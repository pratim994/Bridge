use std::sync::{Arc, Mutex};

use leptos::prelude::*;

use crate::network::client::{GameClient, GameClientError};

use super::{
    commands::{Bid, GameCommand},
    local_client::LocalGameClient,
    reducer::reduce,
    state::{GameState, Seat},
};

#[derive(Clone)]
pub struct GameSession {
    pub state: RwSignal<GameState>,
    pub error: RwSignal<Option<String>>,
    client: Arc<Mutex<Box<dyn GameClient>>>,
}

impl GameSession {
    pub fn local(
        room_id: &str,
        player_name: &str,
        seat: Seat,
        seed: u64,
    ) -> Self {
        Self {
            state: RwSignal::new(GameState::local_room(
                room_id,
                player_name,
                seat,
            )),
            error: RwSignal::new(None),
            client: Arc::new(Mutex::new(Box::new(LocalGameClient::new(seed)))),
        }
    }

    pub fn game_state(&self) -> GameState {
        self.state.get()
    }

    pub fn is_my_turn(&self) -> bool {
        self.state.with(|state| state.is_my_turn())
    }

    pub fn dispatch(
        &self,
        command: GameCommand,
    ) -> Result<(), GameClientError> {
        let current = self.state.get_untracked();
        let events =
            match self.client.lock().unwrap().dispatch(&current, command) {
                Ok(events) => events,
                Err(error) => {
                    self.error.set(Some(error.to_string()));
                    return Err(error);
                }
            };

        let mut next = current;
        for event in events {
            if let Err(error) = reduce(&mut next, event) {
                let error = GameClientError::Other(error.to_string());
                self.error.set(Some(error.to_string()));
                return Err(error);
            }
        }
        self.state.set(next);
        self.error.set(None);
        Ok(())
    }

    pub fn start_game(&self) -> Result<(), GameClientError> {
        self.dispatch(GameCommand::StartGame)
    }

    pub fn play_card(&self, card: super::Card) -> Result<(), GameClientError> {
        self.dispatch(GameCommand::PlayCard { card })
    }

    pub fn place_bid(&self, bid: Bid) -> Result<(), GameClientError> {
        self.dispatch(GameCommand::PlaceBid { bid })
    }
}
