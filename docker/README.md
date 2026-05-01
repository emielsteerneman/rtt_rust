# SSL services — local docker setups

Three tiered compose files for running RoboCup SSL infrastructure locally.
Pick the smallest tier that gives you what you need.

| Tier       | File                  | Services                                                                                          | Use case                                  |
| ---------- | --------------------- | ------------------------------------------------------------------------------------------------- | ----------------------------------------- |
| `minimal`  | `compose.minimal.yml` | ER-Force simulator, TIGERs autoref (passive, as tracker), vision-client                           | Just want tracked frames flowing.         |
| `standard` | `compose.standard.yml`| + game-controller, + simulation-controller. TIGERs autoref switched to active mode.               | Run scrimmages with proper game state.    |
| `full`     | `compose.full.yml`    | + status-board, + ER-Force autoref, + quality-inspector.                                          | Tournament-like setup.                    |

## Run

```bash
docker compose -f docker/compose.minimal.yml up
docker compose -f docker/compose.standard.yml up
docker compose -f docker/compose.full.yml up
```

## Web UIs (host networking)

| Service             | URL                       |
| ------------------- | ------------------------- |
| ssl-vision-client   | http://localhost:8082     |
| ssl-game-controller | http://localhost:8081     |
| ssl-status-board    | http://localhost:8083     |

## Multicast addresses

These compose files use `network_mode: host`, so anything running on the host
(like `rtt_observer`) just sees the multicast traffic directly.

| Channel  | Address              |
| -------- | -------------------- |
| Vision   | `224.5.23.2:10020`   |
| Tracker  | `224.5.23.2:10010`   |
| Referee  | `224.5.23.1:10003`   |

## Notes

- **Linux only.** `network_mode: host` does not work on macOS/Windows Docker
  Desktop. For those, switch to bridge networking and adapt routing — see the
  upstream reference at
  [RoboCup-SSL/ssl-simulation-setup](https://github.com/RoboCup-SSL/ssl-simulation-setup/blob/master/docker-compose.yaml).
- **TIGERs autoref as tracker.** In the minimal tier we drop `-a`. The tracker
  module still emits `TrackerWrapperPacket` on the tracker channel; it just
  doesn't try to push events to a (nonexistent) game-controller.
- **Pinned versions.** Image tags are pinned. To bump, check Docker Hub
  (`robocupssl/*`, `tigersmannheim/auto-referee`, `roboticserlangen/*`)
  and update each file.
- **Simulator choice.** We use ER-Force's headless `simulatorcli` rather than
  grSim. It runs without a display server (better for CI / servers) and is
  what the upstream tournament setup defaults to. `GEOMETRY` and `REALISM`
  env vars pick the field/physics preset.
