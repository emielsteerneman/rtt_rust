use networking::UdpHandler;
use protos::messages::SslWrapperPacket;

pub struct SSLVisionReceiver {
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

