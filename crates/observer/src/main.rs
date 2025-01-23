use networking::UdpHandler;
use protos::messages::SslWrapperPacket;

struct SSLVisionReceiver {
    handler: UdpHandler
}

impl SSLVisionReceiver {
    pub fn new(port: u16) -> Self {
        SSLVisionReceiver { handler: UdpHandler::new(port) }
    }

    pub fn run(&mut self) {
        loop {
            // Receive packets from the network
            let message = self.handler.listen::<SslWrapperPacket>();
            if let Some(message) = message {
                println!("D: {:?}; G: {:?}", message.detection.is_some(), message.geometry.is_some());
            }
        }
    }
}

fn main() {
    let port:u16 = 22222;
    println!("Starting observer on port {}", port);

    // Kinda sucks that this needs to be mutable, all because of the stupid
    // UdpHandler::send_buf having to be mutable in the stupid prost::message::Message::encode method.
    let mut a = SSLVisionReceiver::new(port);
    std::thread::spawn(move || a.run());

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
