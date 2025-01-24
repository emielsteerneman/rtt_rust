use networking::UdpHandler;
use protos::messages::SslWrapperPacket;

struct SSLVisionReceiver {
    handler: UdpHandler,
}

impl SSLVisionReceiver {
    pub fn new(port: u16) -> Self {
        SSLVisionReceiver {
            handler: UdpHandler::new(port),
        }
    }

    pub fn run(&mut self) {
        loop {
            // Receive packets from the network
            match self.handler.listen::<SslWrapperPacket>() {
                Ok(Some(message)) => {
                    tracing::info!(
                        "D: {:?}; G: {:?}",
                        message.detection.is_some(),
                        message.geometry.is_some()
                    );
                }
                Ok(None) => {
                    // No packet received
                }
                Err(e) => {
                    tracing::error!("Error receiving packet: {}", e);
                }
            }
        }
    }
}

fn main() {
    tracing_subscriber::fmt::init();
    color_eyre::install().expect("Failed to install color_eyre");

    let port: u16 = 22222;
    tracing::info!("Starting observer on port {}", port);

    // Kinda sucks that this needs to be mutable, all because of the stupid
    // UdpHandler::send_buf having to be mutable in the stupid prost::message::Message::encode method.
    let mut a = SSLVisionReceiver::new(port);
    std::thread::spawn(move || a.run());

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
