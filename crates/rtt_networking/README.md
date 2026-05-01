# rtt_networking

UDP multicast handler for SSL Vision / Referee streams.

## Design decisions

- **`socket2` over `tokio::UdpSocket` directly** — needed to set `SO_REUSEADDR` before `bind`, which tokio's API doesn't expose.
- **`SO_REUSEADDR`** — multicast is multi-consumer by design; lets us run alongside the ssl-vision GUI, autoref, etc. on the same host without bind conflicts.
- **`join_multicast_v4`** — required. Without the IGMP join the kernel drops multicast packets before they reach the socket.
- **Bind to `0.0.0.0:port`, not `group:port`** — so we also accept unicast on that port (`nc -u 127.0.0.1 <port>` for local testing). Trade-off: socket sees stray unicast too, but the protobuf decode filters that out.
- **No interface selection** — `Ipv4Addr::UNSPECIFIED` lets the kernel pick. The `eth*`/`en*` heuristic from dies-ssl-client would break our docker setup (multicast comes via the docker bridge). Add explicit selection later when deploying on a robot.
