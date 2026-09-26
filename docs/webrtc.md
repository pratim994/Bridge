
# WebRTC Specification

## Purpose

The frontend supports real-time audio/video communication between players.

## Separation of Concerns

Game state and media transport are separate systems.

WebSocket/networking:

- game commands
- game events
- chat
- WebRTC signaling

WebRTC:

- audio
- video
- peer-to-peer media

## Signaling

WebRTC requires signaling to establish peer connections.

Signaling messages are exchanged through the application's existing network transport.

The signaling layer does not carry the actual media stream.

## Peer Connections

Each participant may maintain peer connections with the other participants according to the selected topology.

The initial implementation should prioritize correctness and simplicity for four-player games.

## Media

The browser provides local microphone/camera streams through browser media APIs.

The frontend is responsible for:

- requesting permissions;
- managing local streams;
- displaying local video;
- managing remote streams;
- muting/unmuting;
- enabling/disabling camera;
- cleaning up streams.

## Current implementation status

The frontend provides a local media controller and UI for requesting the local
camera/microphone, muting, toggling the camera, and stopping tracks. Permission
failures are shown in the UI. Remote tiles explicitly show unavailable status.
There are no peer connections or remote streams. The serializable signaling
interface and local queue are scaffolding only; their message shape is not an
agreed server contract and is not connected to a transport. A backend signaling
contract and peer lifecycle design are required before remote media can work.

## TURN/STUN

ICE server configuration must not be hard-coded with production secrets.

Production TURN credentials must be supplied through an appropriate secure configuration mechanism.

## Lifecycle

Media resources must be released when:

- leaving a game;
- disconnecting;
- disabling camera/microphone;
- closing a peer connection.

## Failure Handling

WebRTC failure must not terminate the game.

The game should remain playable if:

- camera permissions are denied;
- microphone permissions are denied;
- a peer connection fails;
- a remote player has no camera;
- media temporarily disconnects.

## Privacy

The frontend must clearly communicate camera/microphone state.

No media should be transmitted without the user's browser permission.
