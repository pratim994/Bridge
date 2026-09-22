use super::{
    events::GameEvent,
    state::GameState,
};

pub fn reduce(
    state: &mut GameState,
    event: GameEvent,
) {
    match event {
        GameEvent::GameState {
            version,
            ..
        } => {
            state.version = version;
        }

        GameEvent::CardPlayed {
            version,
            ..
        } => {
            state.version = version;
        }

        GameEvent::PlayerJoined {
            version,
            ..
        } => {
            state.version = version;
        }

        GameEvent::PlayerLeft {
            version,
            ..
        } => {
            state.version = version;
        }

        GameEvent::Error { .. } => {}
    }
}