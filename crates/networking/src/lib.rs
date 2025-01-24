use std::net::UdpSocket;

use prost::Message;

pub struct UdpHandler {
    pub socket: UdpSocket,
    send_buf: Vec<u8>,
}

impl UdpHandler {
    // Probably return a Result<()> here too
    pub fn new(port: u16) -> Self {
        let addr = format!("0.0.0.0:{}", port);
        tracing::info!("Subscring to UDP on {addr}");

        let socket = UdpSocket::bind(addr).expect("Failed to bind to the port");

        // socket
        //     .set_nonblocking(true)
        //     .expect("Could not set non-blocking");

        let timeout = std::time::Duration::from_secs(2);
        socket
            .set_read_timeout(Some(timeout))
            .expect("Failed to set read timeout");

        UdpHandler {
            socket,
            send_buf: Vec::with_capacity(4096),
        }
    }

    pub fn listen<T: Message + Default>(&self) -> anyhow::Result<Option<T>> {
        let mut buf = [0; 4096];

        Ok(match self.socket.recv_from(&mut buf) {
            Ok((size, _src)) => Some(T::decode(&buf[..size])?),
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => None,
            Err(e) => {
                tracing::error!("Error receiving data: {}", e);
                anyhow::bail!("Error receiving data: {}", e)
            }
        })
    }

    pub fn send<T: Message>(&mut self, msg: T) -> anyhow::Result<()> {
        // Localhost ToSocketAddrs implementation
        // I don't know what this port is for Emiel, but I'm just going to assume it's correct.
        // where are you sending this shit? Aren't you just using `.listen()` by itself? I don't see .send() called
        let addr: &str = "127.0.0.1:8010";

        // Place message in send_buf. We're just going to assume this works.
        msg.encode(&mut self.send_buf)?;

        // Send message. Also assuming this simply works.
        self.socket.send_to(&self.send_buf, addr)?;

        Ok(())
    }
}
