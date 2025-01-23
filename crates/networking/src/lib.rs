use std::net::UdpSocket;

use prost::Message;

pub struct UdpHandler {
    pub socket: UdpSocket,
    send_buf: Vec<u8>
}

impl UdpHandler {
    
    pub fn new(port: u16) -> Self {
        let socket = UdpSocket::bind(format!("0.0.0.0:{}", port))
            .expect("Failed to bind to the port");

        // socket
        //     .set_nonblocking(true)
        //     .expect("Could not set non-blocking");

        let timeout = std::time::Duration::from_secs(2);
        socket.set_read_timeout(Some(timeout)).expect("Failed to set read timeout");

        UdpHandler { socket, send_buf: Vec::with_capacity(4096) }
    }

    pub fn listen<T: Message + Default>(&self) -> Option<T> {
        let mut buf = [0; 4096];

        match self.socket.recv_from(&mut buf) {
            Ok((size, _src)) => {
                T::decode(&buf[..size]).ok()
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => None,
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                None
            }
        }
    }

    pub fn send<T: Message>(&mut self, msg:T) {

        // Localhost ToSocketAddrs implementation
        let addr:&str = "127.0.0.1:8010";

        // Place message in send_buf. We're just going to assume this works.
        msg.encode(&mut self.send_buf).expect("Failed to encode message");

        // Send message. Also assuming this simply works.
        self.socket.send_to(&self.send_buf, addr).expect("Failed to send message");

    }

}