# Frontend Engineering Documentation

This directory contains the engineering specifications for the Rust/WebAssembly bridge frontend.

## Documents

### Architecture

- [Architecture](architechture.md)
- [Game State](game-state.md)
- [Networking](networking.md)
- [WebRTC](webrtc.md)

### Domain

- [Bridge Rules](bridge-rules.md)

### Engineering

- [Milestones](milestones.md)

The repository's frontend documentation currently lives in this `docs/` directory;
there is no `frontend/docs/` directory. Testing commands and asset layout are
documented in the frontend README and Trunk configuration.

### Architecture Decisions

Architecture decisions are stored under:

`decisions/`

## Source of Truth

The documentation describes intended architecture and behavior.

The actual source code remains the implementation.

If documentation and implementation disagree, the discrepancy should be identified rather than silently ignored.

## Updating Documentation

Update documentation when:

- a protocol changes;
- a state model changes;
- a major architectural decision is made;
- a new frontend subsystem is introduced;
- an existing behavior changes in a way that affects future development.
