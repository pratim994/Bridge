# Frontend Development Milestones

## Phase 1 — Foundation

- [x] Rust/WASM project
- [x] Leptos application
- [x] Trunk build
- [x] Static card assets
- [x] Card model
- [x] Card rendering

## Phase 2 — Local Table

- [x] Player hand
- [x] Card hover/interaction
- [x] Play card interaction
- [x] Played card moved to center
- [x] Opponent rendering (public card counts only)
- [x] Partner rendering
- [x] Seat-aware table layout
- [x] Turn indicator
- [x] Trick positioning

## Phase 3 — Game State

- [x] Formal GameState
- [x] GamePhase
- [x] GameCommand
- [x] GameEvent
- [x] Reducer
- [x] State versioning
- [x] Deterministic reducer tests
- [x] Bidding state
- [x] Contract state
- [x] Trick state
- [x] Score state

## Phase 4 — Multiplayer Transport

- [ ] WebSocket connection
- [ ] Authentication
- [x] Serializable frontend command/event types (not a backend wire contract)
- [x] Event deserialization types
- [ ] Connection state
- [ ] Reconnection
- [ ] State synchronization
- [ ] Protocol error handling

## Phase 5 — Chat

- [x] Chat messages (local mock client)
- [x] Chat history (in-memory local session)
- [x] Chat UI
- [ ] Server event integration

## Phase 6 — WebRTC

- [x] Local media
- [x] Video tiles (local stream and explicit unavailable remote tiles)
- [ ] Peer connections
- [ ] Signaling
- [ ] ICE handling
- [x] Mute/unmute
- [x] Camera toggle
- [x] Connection failure handling

## Phase 7 — Polish

- [x] Seat rotation
- [ ] Card animations
- [ ] Trick animations
- [x] Responsive layout
- [x] Connection indicators (local mode)
- [x] Accessibility basics
- [x] Loading/error states

## Implementation status

The frontend runs a deterministic local four-player demo. The local adapter privately
holds simulated opponent hands; the public `GameState` exposes only the local hand
and opponents' remaining card counts. Bidding and trick play are implemented.

Backend WebSocket synchronization, chat transport, peer connections, remote media,
and production deployment remain unimplemented because the repository does not
specify the required backend message, signaling, authentication, or deployment
contracts. Frontend interfaces and local development behavior are available where
useful. The existing low-level WebSocket draft is not connected to the application.

## Phase 8 — Production

- [ ] Production build
- [ ] CDN/static deployment
- [ ] Browser integration tests
- [ ] Performance profiling
- [ ] Error monitoring
- [ ] Production WebRTC configuration
