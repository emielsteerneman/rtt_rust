// System libraries
use std::sync::{Arc, Mutex};

// Local libraries
use protos::messages::{SslDetectionFrame, SslGeometryData};

// mod ssl_receivers;
// use ssl_receivers::SSLVisionReceiver;

mod processor;
use processor::Processor;

struct LatestData {
    detection: Option<SslDetectionFrame>,
    geometry: Option<SslGeometryData>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt::init();
    color_eyre::install().expect("Failed to install color_eyre");


    // Create a shared data structure to store the latest data
    let latest_data = Arc::new(Mutex::new(LatestData {
        detection: None,
        geometry: None,
    }));


    let port_ssl_vision: u16 = 22222;
    tracing::info!("Starting observer on port {}", port_ssl_vision);

    // Kinda sucks that this needs to be mutable, all because of the stupid
    // UdpHandler::send_buf having to be mutable in the stupid prost::message::Message::encode method.
    let mut a = SSLVisionReceiver::new(port_ssl_vision);
    std::thread::spawn(move || a.run());





    let processor: Processor = Processor::new(1.0);
    std::thread::spawn(move || processor.run());


    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
