use std::net::{Ipv4Addr, SocketAddrV4};

use prost::Message;
use socket2::{Domain, Protocol, Socket, Type};
use tokio::net::UdpSocket;

pub struct UdpHandler {
    socket: UdpSocket,
    send_buf: Vec<u8>, // <- Can this not be a fixed size? A simple buffer like we have in listen()?
}

impl UdpHandler {
    pub async fn new(group: Ipv4Addr, port: u16) -> anyhow::Result<Self> {
        tracing::info!("Subscribing to multicast {group}:{port}");

        // Drop down to socket2 because tokio's UdpSocket doesn't let us set
        // SO_REUSEADDR before bind, which we need for multicast.
        let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;

        // tokio drives sockets via its async reactor, which requires the
        // underlying fd to be in non-blocking mode.
        socket.set_nonblocking(true)?;

        // Multicast is meant for multiple consumers on the same host (e.g.
        // observer + ssl-vision GUI). Without REUSEADDR the second process to
        // start would fail to bind the same group:port.
        socket.set_reuse_address(true)?;

        // IGMP join: tells the kernel (and any IGMP-snooping switch) that this
        // host wants packets for `group`. Without this, multicast packets are
        // dropped before they ever reach the socket.
        socket.join_multicast_v4(&group, &Ipv4Addr::UNSPECIFIED)?;

        // Bind to 0.0.0.0 rather than the group address so we also accept
        // unicast on this port (e.g. `nc -u 127.0.0.1 <port>` for local tests).
        socket.bind(&SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port).into())?;

        // Hand the configured fd off to tokio so we can `await` on it.
        let socket = UdpSocket::from_std(socket.into())?;

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
            }
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
