# AGENTS.md — GameServer

## Purpose

This repository contains a multiplayer trick-taking bridge game.

The system consists of multiple independently deployable areas, including:

- Rust/WebAssembly frontend
- frontend game-state and presentation logic
- Django/DRF backend
- serverless/AWS infrastructure
- multiplayer networking
- WebRTC communication

## Codex Scope

Codex is primarily authorized to work on:

- `frontend/`
- Rust/WASM frontend code
- frontend game-state management
- frontend components
- frontend networking client code
- frontend WebRTC client code
- frontend tests
- frontend documentation

Unless explicitly instructed otherwise, Codex MUST NOT modify:

- `backend/`
- Django/DRF application code
- database schemas or migrations
- AWS infrastructure
- Lambda configuration
- API Gateway configuration
- deployment configuration outside `frontend/`
- production credentials or secrets
- authentication infrastructure

If a frontend task appears to require backend changes, stop and explain the required backend contract instead of modifying the backend.

## Repository Principles

Prefer small, reviewable changes.

Before changing existing architecture:

1. Inspect the existing implementation.
2. Read the relevant documentation under `frontend/docs/`.
3. Determine whether the requested behavior already exists.
4. Reuse existing abstractions where appropriate.
5. Avoid unnecessary rewrites.

Do not replace working code merely to introduce a preferred architectural style.

## Authority Boundary

The backend is authoritative for multiplayer game state.

The frontend is responsible for:

- presenting game state
- collecting user actions
- maintaining a client-side projection of authoritative state
- rendering animations and transitions
- displaying communication state
- managing browser-side WebRTC functionality

The frontend MUST NOT become the authoritative source of multiplayer game state.

Client-side validation may improve UX, but it must never be treated as a security or cheating-prevention mechanism.

## Hidden Information

Never expose another player's private cards to the client merely for rendering convenience.

A player's client may receive:

- its own hand
- public game information
- public trick information
- public bidding information
- permitted player metadata

Opponent private hands must remain hidden.

## Documentation

Frontend architectural and behavioral documentation lives under:

`frontend/docs/`

Relevant documentation should be updated when an architectural decision or externally visible behavior changes.

## Testing

A task is not considered complete merely because the implementation compiles.

Run the most relevant validation available for the change.

For Rust frontend changes, normally use:

- `cargo check`
- `cargo test`

For build-related changes, also use:

- `trunk build`

If a command cannot be run, state why.

## Completion Report

When completing a task, report:

1. What changed.
2. Which files changed.
3. Tests/checks executed.
4. Test/check results.
5. Any assumptions.
6. Any unresolved issues.

## Conflict Handling

If repository code and documentation disagree:

- inspect the implementation;
- identify the discrepancy;
- do not silently invent a new behavior;
- explain the conflict before making a large architectural change.

If a task is ambiguous and the ambiguity affects architecture, stop and ask for clarification.
