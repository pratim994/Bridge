use leptos::prelude::*;

use super::{Card, GameState, PlayedCard, Seat, Turn};

#[derive(Clone, Copy)]
pub struct GameSession {
    pub state: RwSignal<GameState>,
}

impl GameSession {
    pub fn new(state: GameState) -> Self {
        Self {
            state: RwSignal::new(state),
        }
    }

    pub fn game_state(&self) -> GameState {
        self.state.get()
    }

    pub fn is_my_turn(&self) -> bool {
        self.state.with(|state| state.is_my_turn())
    }

    pub fn play_card(&self, card: Card) {
        self.state.update(|state| {
            if state.current_turn.seat() != state.my_seat {
                return;
            }

            let Some(index) = state.my_hand.iter().position(|hand_card| {
                *hand_card == card
            }) else {
                return;
            };

            state.my_hand.remove(index);

            state.current_trick.push(PlayedCard {
                player_id: player_id_for_seat(state.my_seat),
                card,
            });

            state.version += 1;

            state.current_turn = next_turn(state.current_turn);
        });
    }

    pub fn reset_demo(&self) {
        self.state.set(GameState::demo());
    }
}

fn player_id_for_seat(seat: Seat) -> String {
    match seat {
        Seat::North => "north",
        Seat::East => "east",
        Seat::South => "south",
        Seat::West => "west",
    }
    .to_string()
}

fn next_turn(turn: Turn) -> Turn {
    match turn {
        Turn::North => Turn::East,
        Turn::East => Turn::South,
        Turn::South => Turn::West,
        Turn::West => Turn::North,
    }
}