use tokio::net::UdpSocket;

use prost::Message;

pub struct UdpHandler {
    socket: UdpSocket,
    send_buf: Vec<u8>, // <- Can this not be a fixed size? A simple buffer like we have in listen()?
}

impl UdpHandler {
    pub async fn new(port: u16) -> anyhow::Result<Self> {
        let addr = format!("0.0.0.0:{}", port);
        tracing::info!("Subscring to UDP on {addr}");

        let socket = UdpSocket::bind(addr).await?;

        Ok(Self {
            socket,
            send_buf: Vec::with_capacity(4096),
        })
    }

    pub async fn listen<T: Message + Default>(&self) -> anyhow::Result<T> {
        let mut buf = [0; 4096]; // <- Can we do this once somewhere, and not every time we call listen?
        
        // Receive a message from the socket
        Ok(match self.socket.recv_from(&mut buf).await {
            // A message was successfully received
            Ok((size, _src)) => {
                // Attempt to decode the message into an instance of T
                match T::decode(&buf[..size]) {
                    // If the message was successfully decoded, return it
                    Ok(msg) => msg,
                    // If the message could not be decoded, return Error
                    Err(_) => anyhow::bail!("Could not decode message"),
                }
            },
            // An error occurred while receiving the message
            Err(e) => {
                tracing::error!("Error receiving data: {}", e);
                anyhow::bail!("Error receiving data: {}", e)
            }
        })
    }

    // For when we want to send stuff to RoboTeam AI. Yet to be used.
    pub async fn send<T: Message>(&mut self, msg: T) -> anyhow::Result<()> {
        let addr: &str = "127.0.0.1:8010";

        // Place message in send_buf. We're just going to assume this works.
        msg.encode(&mut self.send_buf)?;

        // Send message. Also assuming this simply works.
        self.socket.send_to(&self.send_buf, addr).await?;

        Ok(())
    }
}
