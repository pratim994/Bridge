
# Game State Specification

## Purpose

This document defines the conceptual model for the client-side representation of a bridge game.

The frontend state is a projection of authoritative server state.

It is not a replacement for the backend game engine.

## Game Lifecycle

A game progresses conceptually through:

1. Lobby
2. Players seated
3. Deal
4. Bidding
5. Contract established
6. Card play
7. Trick completion
8. Hand completion
9. Scoring
10. Game completion

The exact state machine is implemented by the game-state module.

The current client has `Lobby`, `Dealing`, `Bidding`, `Playing`, `HandComplete`,
and `Complete` phases. A deterministic seeded deck deals cards round-robin to
four seats. `Hand` and `Trick` are pure Rust domain types, and the reducer applies
versioned serializable events without depending on Leptos or browser APIs.

`LocalGameClient` is the current executable adapter. It owns opponent hands
privately, deals a local hand into the public projection, runs pass bids for the
simulated opponents, and plays deterministic legal bot cards. It is a local demo,
not an authoritative multiplayer engine. A contract and per-hand trick counts are
represented; duplicate/rubber scoring is not implemented.

## GameState

GameState should represent the information required to render the current game.

Conceptually this includes:

- players
- local player identity
- local seat
- local hand
- game phase
- current turn
- current trick
- bidding state
- contract
- score
- state/version information

The implementation may contain additional fields.

## Player

A player should contain information required to identify and render that player.

Conceptually:

- player ID
- display name
- seat
- connection status
- public card count

A player's private hand must not be represented on another player's client.

## Seats

The four bridge seats are:

- North
- East
- South
- West

The local player may occupy any seat.

The UI should eventually rotate the table so the local player is always presented as the bottom/player-facing seat.

## Hand

The local player has access to their own cards.

A standard bridge hand contains 13 cards.

The frontend should preserve the card model rather than representing cards as arbitrary UI strings.

## Trick

A trick contains cards played by the four players during the current trick.

Each played card must retain enough information to determine:

- card identity
- player who played it
- seat/position

The UI renders cards according to their player's position around the table.

## Turn

Only the player whose turn it is may make the corresponding game action.

The frontend may disable controls for other players.

The backend remains authoritative.

## Game Phase

The frontend must distinguish at minimum between:

- lobby
- bidding
- playing
- complete

Additional phases may be introduced as required.

## Commands

Commands represent user intent.

Examples:

- play card
- place bid
- pass
- double
- redouble
- send chat
- send WebRTC signaling message

A command does not mean the action succeeded.

## Events

Events represent state changes accepted by the authoritative backend.

Examples:

- game started
- bid placed
- card played
- turn changed
- trick completed
- score changed
- chat received
- signaling message received

## Reducer

The reducer transforms existing state in response to events.

Conceptually:

    previous_state + event -> next_state

Reducer logic should be deterministic.

The reducer should not:

- perform network requests;
- access browser APIs;
- mutate UI components;
- make authoritative game decisions.

## State Versioning

State synchronization should use a version or equivalent ordering mechanism.

The client must not blindly apply stale authoritative state.

The exact protocol is defined in `networking.md`.

There is not yet a backend protocol implementation. The versioned event model and
the `GameClient` interface are ready for an adapter, but the current websocket
module is not connected and its placeholder server message type is not a contract.

## Hidden Information

The client must never receive private information about other players merely because the UI could use it.

The local hand is private.

Opponent hands are represented only through permitted public information such as card count.

## Optimistic UI

Optimistic updates should be used cautiously.

For authoritative game actions such as playing a card or bidding, the preferred model is:

    user action
        |
        v
    command
        |
        v
    server validation
        |
        v
    authoritative event
        |
        v
    reducer
        |
        v
    UI update

If optimistic UI is introduced, rollback behavior must be explicitly designed and tested.
