# Frontend Engineering Instructions

## Scope

This directory contains the Rust/WebAssembly frontend for the multiplayer bridge game.

The frontend is responsible for:

- bridge table presentation
- card rendering
- player presentation
- local UI state
- client-side game-state projection
- user interaction
- game commands
- server event handling
- WebSocket client behavior
- WebRTC browser behavior
- chat presentation
- frontend tests

The frontend is NOT authoritative for multiplayer game rules.

## Technology

The frontend currently uses:

- Rust
- WebAssembly
- Leptos
- Trunk
- browser APIs through `web-sys` / `wasm-bindgen` where appropriate

Use the versions already declared by `Cargo.toml`.

Do not upgrade dependencies unless the task explicitly requires it.

## Project Structure

The expected conceptual organization is:

- `src/components/` — UI components
- `src/game/` — game models, state, commands, events and reducers
- `src/network/` — networking client/protocol functionality
- `src/webrtc/` — browser WebRTC functionality
- `src/api/` — API client functionality
- `public/` — static frontend assets
- `docs/` — frontend engineering documentation

The actual repository structure takes precedence over this conceptual description.

## Component Rules

Components should primarily render state and dispatch user intent.

Avoid putting authoritative bridge rules directly inside UI components.

Do not duplicate game-state logic between multiple components.

Prefer reusable components for:

- cards
- hands
- players
- tricks
- bidding
- scoreboard
- chat
- video participants

## Game-State Rules

Game state should have a clear distinction between:

- authoritative state received from the server
- local UI state
- user commands
- server events

Prefer the following conceptual flow:

User interaction
    ↓
GameCommand
    ↓
network transport
    ↓
authoritative backend
    ↓
GameEvent
    ↓
frontend reducer
    ↓
GameState
    ↓
UI

Do not bypass this boundary without a documented reason.

## Card Rules

Cards are represented by the existing game model.

Card rendering must use the existing card asset conventions.

Static card assets are served from the frontend's public asset directory.

Do not embed large card images directly into Rust source.

Do not replace the existing PNG card assets with another format unless explicitly requested.

Opponent cards must be represented by card backs or other hidden-card UI.

## Bridge Rules

The frontend may use bridge rules for:

- visual feedback
- disabling obviously invalid actions
- displaying bidding controls
- displaying turn information

The backend remains authoritative for:

- legal bids
- legal card plays
- ownership of cards
- following suit
- trick winners
- contracts
- scoring
- game completion

Never rely on frontend validation for security.

## Networking

The frontend communicates with the authoritative backend through the project's defined network protocol.

Do not invent a new protocol when an existing protocol is present.

Before modifying network messages:

1. Read `docs/networking.md`.
2. Inspect the existing protocol implementation.
3. Determine whether the change is backwards compatible.
4. Document protocol changes.

## WebRTC

WebRTC is responsible for real-time media such as:

- microphone
- camera
- peer connections

WebSocket/network messaging is used for signaling.

Do not send game state or media through WebRTC data channels unless explicitly specified.

Do not place backend media infrastructure inside the frontend implementation.

## State Synchronization

The frontend must tolerate:

- delayed events
- duplicated events where the protocol permits them
- reconnects
- stale state
- temporary network failure

State versioning and synchronization behavior are defined in:

`docs/game-state.md`

and

`docs/networking.md`

## Error Handling

Do not silently swallow network, serialization, or state-transition errors.

Errors should be:

- represented explicitly where possible;
- logged appropriately during development;
- surfaced to the UI when they affect the user.

Avoid panic-driven control flow in browser-facing code.

## Testing

Game-state behavior should have deterministic tests.

Prioritize tests for:

- card removal
- card play
- trick progression
- turn progression
- bidding
- state transitions
- event reduction
- serialization/deserialization
- invalid state transitions

UI tests should be added where they provide meaningful coverage.

## Build Validation

At minimum, relevant Rust changes should pass:

`cargo check`

Game-state changes should normally pass:

`cargo test`

Frontend build changes should normally pass:

`trunk build`

Do not claim success without actually running the applicable validation.

## Change Discipline

Prefer:

- small diffs
- existing abstractions
- explicit types
- deterministic state transitions
- testable pure functions

Avoid:

- speculative abstractions
- unnecessary dependencies
- massive component rewrites
- duplicated state
- hidden global mutable state
- mixing networking, game rules and presentation unnecessarily

## Generated Code

Do not commit generated build output unless the repository explicitly requires it.

Normally do not modify:

- `dist/`
- generated WASM
- generated JavaScript
- generated CSS

unless the project's deployment process explicitly requires tracked build artifacts.

## Secrets

Never commit:

- API keys
- WebRTC TURN credentials
- JWT secrets
- AWS credentials
- private keys
- tokens
- `.env` files containing secrets
