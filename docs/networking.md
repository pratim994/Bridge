# Frontend Networking Specification

## Purpose

This document defines the frontend's relationship with the authoritative multiplayer backend.

## Transport

The multiplayer game uses a WebSocket-based transport for real-time events.

The frontend does not host or own the authoritative WebSocket server.

## Current implementation status

The UI talks to `GameSession`, which dispatches commands through a `GameClient`
interface. `LocalGameClient` implements that interface for the playable local demo.
Game commands/events are serde-serializable, but their representation is not an
agreed backend wire format. A browser websocket wrapper exists as an unused draft;
there is no endpoint, authentication flow, payload mapping, event stream, or
reconnection integration. Do not connect it until those contracts are specified.

Chat currently uses a local in-memory client. No chat transport adapter is wired.

## Responsibilities

The frontend network layer is responsible for:

- establishing connections;
- authenticating where required;
- serializing commands;
- sending commands;
- receiving events;
- deserializing events;
- handling disconnects;
- reconnecting where appropriate;
- exposing connection state to the UI.

## Commands

Commands represent requests from the client.

Examples:

- play a card;
- place a bid;
- pass;
- double;
- redouble;
- send chat;
- send signaling information.

Commands are requests, not authoritative state changes.

## Events

Events represent accepted state changes or server-originated information.

The frontend reducer consumes game-state events.

## Authority

The backend validates all authoritative game actions.

The frontend must not assume that a sent command succeeded.

## Reconnection

The client should be able to recover from temporary connection loss.

Reconnect behavior should use bounded exponential backoff.

The client must not create uncontrolled reconnect loops.

## Synchronization

After reconnecting, the client should obtain authoritative state or the appropriate event history before allowing the local projection to be considered synchronized.

## Ordering

Events must be processed according to the ordering guarantees provided by the backend protocol.

If the protocol provides sequence numbers or state versions, the frontend must use them.

## Duplicate Events

If the protocol permits duplicate delivery, the frontend must handle duplicate events safely.

Reducers should be designed so that duplicate processing does not silently corrupt state.

## Invalid Messages

Malformed or unknown messages should not crash the application.

They should be logged appropriately and handled according to the protocol's error policy.

## Protocol Changes

Any externally visible protocol change must be documented before implementation.

Do not invent backend message formats without agreement with the backend implementation.

## Backend Boundary

If a frontend feature requires a new backend capability:

1. describe the required command/event;
2. document the expected payload;
3. identify authentication/authorization requirements;
4. identify state/version implications;
5. do not modify backend code unless explicitly authorized.
