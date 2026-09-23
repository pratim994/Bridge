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

GameEvent::CardPlayed {
    player_id,
    card,
} => {
    if player_id == &my_player_id {
        state.my_hand.retain(|c| *c != card);
    }

    state.current_trick.push(PlayedCard {
        player_id: player_id.clone(),
        card: *card,
    });
}