// System libraries
use std::cell::RefCell;

// Local libraries
use protos::messages::Referee;
use protos::messages::{SslDetectionFrame, SslGeometryData, SslWrapperPacket};

use tokio::time::Duration;

use networking::UdpHandler;

struct LatestData {
    detection: RefCell<Option<SslDetectionFrame>>,
    geometry: RefCell<Option<SslGeometryData>>,
    referee: RefCell<Option<Referee>>,
}

async fn handle_receive_vision(
    latest_data: &LatestData,
    wrapper_packet: anyhow::Result<SslWrapperPacket>,
) {
    match wrapper_packet {
        Ok(wrapper_packet) => {
            tracing::info!("Received detection frame");
            // Store Detection frame if present
            if wrapper_packet.detection.is_some() {
                *latest_data.detection.borrow_mut() = wrapper_packet.detection.clone();
            }
            // Store Geometry frame if present
            if wrapper_packet.geometry.is_some() {
                *latest_data.geometry.borrow_mut() = wrapper_packet.geometry.clone();
            }
        }
        Err(e) => {
            tracing::error!("Error receiving detection frame: {}", e);
        }
    }
}

async fn handle_receive_referee(latest_data: &LatestData, referee_packet: anyhow::Result<Referee>) {
    match referee_packet {
        Ok(referee_packet) => {
            tracing::info!("Received referee packet");

            // Store Referee packet if present
            *latest_data.referee.borrow_mut() = Some(referee_packet).clone();
            // NOTE We're first unpacking and then repacking the referee packet.
            // Might be useless, we can just match on Ok(referee_packet). However, this function
            // is only triggered if a referee packet is received, so it should always be Some. If
            // it's None, it indicates a parsing error, which we should log and ignore.
        }
        Err(e) => {
            tracing::error!("Error receiving referee packet: {}", e);
        }
    }
}

fn send_out_data() {
    // Send data to RoboTeam AI
    tracing::info!("Sending data to RoboTeam AI");
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    color_eyre::install().expect("Failed to install color_eyre");

    // Create a shared data structure to store the latest data
    let latest_data = LatestData {
        detection: RefCell::new(None),
        geometry: RefCell::new(None),
        referee: RefCell::new(None),
    };

    let port_ssl_vision: u16 = 22222;
    let port_referee: u16 = 22223;

    let vision_receiver = UdpHandler::new(port_ssl_vision).await?;
    let referee_receiver = UdpHandler::new(port_referee).await?;

    let mut interval = tokio::time::interval(Duration::from_secs_f32(1. / 60.));

    loop {
        // Receive any packets from the SSL Vision or Referee
        tokio::select! {
            // Always try to send out data first
            biased;

            _ = interval.tick() => {
                send_out_data();
                // Since we're here, this branch of tokio::select! will complete first, thus the other
                // branches will be skipped. This means that we will receive no new data, so we can skip the
                // rest of the loop. Go straight to the next loop to receive new data.
                continue;
            }

            wrapper_packet = vision_receiver.listen::<SslWrapperPacket>() => {
                handle_receive_vision(&latest_data, wrapper_packet).await;
            }

            referee_packet = referee_receiver.listen::<Referee>() => {
                handle_receive_referee(&latest_data, referee_packet).await;
            }
        }

        // Process the latest data
        tracing::info!(
            "D:{} G:{} R:{}",
            latest_data.detection.borrow().is_some(),
            latest_data.geometry.borrow().is_some(),
            latest_data.referee.borrow().is_some()
        );
    }
}
