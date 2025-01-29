use tokio::net::UdpSocket;

use prost::Message;

pub struct UdpHandler {
    pub socket: UdpSocket,
    send_buf: Vec<u8>,
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

    pub async fn listen<T: Message + Default>(&self) -> anyhow::Result<Option<T>> {        
        let mut buf = [0; 4096];

        Ok(match self.socket.recv_from(&mut buf).await {
            Ok((size, _src)) => Some(T::decode(&buf[..size])?),
            Err(e) => {
                tracing::error!("Error receiving data: {}", e);
                anyhow::bail!("Error receiving data: {}", e)
            }
        })
    }

    // pub async fn 

    // For when we want to send stuff to RoboTeam AI
    pub async fn send<T: Message>(&mut self, msg: T) -> anyhow::Result<()> {

        let addr: &str = "127.0.0.1:8010";

        // Is this needed?
        self.send_buf.clear();

        // Place message in send_buf. We're just going to assume this works.
        msg.encode(&mut self.send_buf)?;

        // Send message. Also assuming this simply works.
        self.socket.send_to(&self.send_buf, addr).await?;

        Ok(())
    }
}
