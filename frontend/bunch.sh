for f in \
  Cargo.toml \
  Trunk.toml \
  index.html \
  src/main.rs \
  src/app.rs \
  src/components/mod.rs \
  src/components/table.rs \
  src/components/hand.rs \
  src/components/card.rs \
  src/components/player.rs \
  src/components/trick.rs \
  src/components/scoreboard.rs \
  src/components/chat.rs \
  src/game/mod.rs \
  src/game/state.rs \
  src/game/commands.rs \
  src/game/events.rs \
  src/game/reducer.rs \
  src/network/mod.rs \
  src/network/protocol.rs \
  src/network/websocket.rs \
  src/network/reconnect.rs \
  src/webrtc/mod.rs \
  src/webrtc/peer.rs \
  src/webrtc/manager.rs \
  src/webrtc/signaling.rs \
  src/webrtc/media.rs \
  src/api/mod.rs \
  src/api/client.rs
do
  echo
  echo "==================== $f ===================="
  [ -f "$f" ] && cat "$f" || echo "[missing]"
done
