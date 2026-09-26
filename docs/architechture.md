
# Frontend Architecture

## Purpose

The frontend is a browser-based Rust/WebAssembly application for a multiplayer bridge game.

Its responsibilities are presentation, interaction, client-side state projection, networking, and browser communication capabilities.

## High-Level Architecture

The conceptual architecture is:

Browser
|
+-- Leptos UI
|
+-- GameState projection
|
+-- Game reducer
|
+-- Game commands
|
+-- WebSocket client
|
+-- WebRTC client
|
+-- Browser media APIs
|
+-- Static assets
|
v
Authoritative backend

## State Flow

User interaction produces a command.

Command:

    UI
     |
     v
    GameCommand
     |
     v
    Network transport
     |
     v
    Authoritative backend

The backend produces events.

Event:

    Backend
       |
       v
    GameEvent
       |
       v
    Reducer
       |
       v
    GameState
       |
       v
    UI

## Architectural Boundary

The frontend does not own authoritative multiplayer state.

The backend decides whether a requested action is legal.

The frontend may perform local validation for user experience, but server validation is authoritative.

## UI Layer

Components should primarily:

- consume state;
- render state;
- dispatch user intent.

Components should not contain authoritative game rules.

## Game Layer

The game layer contains:

- card representations;
- player representations;
- seats;
- game phases;
- turns;
- tricks;
- contracts;
- bidding state;
- commands;
- events;
- reducers.

The exact implementation should follow the current source tree.

## Network Layer

The network layer translates between frontend commands/events and the backend protocol.

Transport-specific concerns should not leak unnecessarily into game components.

## WebRTC Layer

WebRTC handles real-time media.

Signaling is handled through the application's signaling transport.

Media transport and game-state transport are separate concerns.

## Static Assets

Static assets are served by the frontend's static asset pipeline.

Card images are not bundled into Rust source as binary data.

## Extensibility

The architecture should allow:

- reconnection;
- multiple simultaneous games;
- player seat rotation;
- future spectator mode;
- chat;
- video/audio;
- server-side authoritative validation;
- game replay or event history.
