use crate::network::client::{GameClient, GameClientError};

use super::{
    Deck, Hand,
    commands::{Bid, GameCommand},
    events::GameEvent,
    reducer::reduce,
    state::{GamePhase, GameState, Seat, local_player_id},
};

/// Deterministic local room adapter. Opponent hands stay inside this adapter and are never
/// copied into the renderable GameState projection.
pub struct LocalGameClient {
    seed: u64,
    hands: Option<[Hand; 4]>,
}

impl LocalGameClient {
    pub fn new(seed: u64) -> Self {
        Self { seed, hands: None }
    }

    fn deal(
        &mut self,
        state: &GameState,
    ) -> Result<Vec<GameEvent>, GameClientError> {
        let hands = Deck::standard().shuffled(self.seed).deal();
        self.seed = self.seed.wrapping_add(1);
        let local_hand = hands[state.my_seat.index()].cards().to_vec();
        self.hands = Some(hands);
        let event = GameEvent::HandDealt {
            version: next_version(state)?,
            hand: local_hand,
            card_counts: [13; 4],
        };
        let mut simulated = state.clone();
        let mut events = Vec::new();
        self.emit(&mut simulated, &mut events, event)?;
        self.advance_bidding(&mut simulated, &mut events)?;
        Ok(events)
    }

    fn advance_bidding(
        &mut self,
        state: &mut GameState,
        events: &mut Vec<GameEvent>,
    ) -> Result<(), GameClientError> {
        while state.phase == GamePhase::Bidding
            && state.current_turn.seat() != state.my_seat
        {
            let seat = state.current_turn.seat();
            let bid = GameEvent::BidPlaced {
                version: next_version(state)?,
                player_id: local_player_id(seat),
                bid: Bid::Pass,
            };
            self.emit(state, events, bid)?;
            if state.bidding.is_complete(state.contract.is_some()) {
                if state.contract.is_some() {
                    let started = GameEvent::GameStarted {
                        version: next_version(state)?,
                    };
                    self.emit(state, events, started)?;
                    self.play_bots_until_local(state, events)?;
                } else {
                    let redeal = self.deal(state)?;
                    for event in redeal {
                        self.emit(state, events, event)?;
                    }
                }
            } else {
                let turn_changed = GameEvent::TurnChanged {
                    version: next_version(state)?,
                    turn: turn_for(seat.next()),
                };
                self.emit(state, events, turn_changed)?;
            }
        }
        Ok(())
    }

    fn place_bid(
        &mut self,
        state: &GameState,
        bid: Bid,
    ) -> Result<Vec<GameEvent>, GameClientError> {
        if state.phase != GamePhase::Bidding
            || state.current_turn.seat() != state.my_seat
        {
            return Err(GameClientError::InvalidAction);
        }
        let mut simulated = state.clone();
        let mut events = Vec::new();
        let bid_event = GameEvent::BidPlaced {
            version: next_version(&simulated)?,
            player_id: local_player_id(state.my_seat),
            bid,
        };
        self.emit(&mut simulated, &mut events, bid_event)?;
        if simulated.bidding.is_complete(simulated.contract.is_some()) {
            if simulated.contract.is_some() {
                let event = GameEvent::GameStarted {
                    version: next_version(&simulated)?,
                };
                self.emit(&mut simulated, &mut events, event)?;
                self.play_bots_until_local(&mut simulated, &mut events)?;
            } else {
                let redeal = self.deal(&simulated)?;
                for event in redeal {
                    self.emit(&mut simulated, &mut events, event)?;
                }
            }
        } else {
            let event = GameEvent::TurnChanged {
                version: next_version(&simulated)?,
                turn: turn_for(state.my_seat.next()),
            };
            self.emit(&mut simulated, &mut events, event)?;
            self.advance_bidding(&mut simulated, &mut events)?;
        }
        Ok(events)
    }

    fn play(
        &mut self,
        state: &GameState,
        card: super::Card,
    ) -> Result<Vec<GameEvent>, GameClientError> {
        if state.phase != GamePhase::Playing || !state.can_play_card(card) {
            return Err(GameClientError::InvalidAction);
        }
        let local_seat = state.my_seat;
        if !self.hand_mut(local_seat)?.remove(card) {
            return Err(GameClientError::InvalidAction);
        }

        let mut next = state.clone();
        let mut events = Vec::new();
        self.emit_card(&mut next, &mut events, local_seat, card)?;
        self.play_bots_until_local(&mut next, &mut events)?;
        Ok(events)
    }

    fn play_bots_until_local(
        &mut self,
        state: &mut GameState,
        events: &mut Vec<GameEvent>,
    ) -> Result<(), GameClientError> {
        let mut steps = 0;
        while state.phase == GamePhase::Playing
            && state.current_turn.seat() != state.my_seat
            && steps < 52
        {
            let seat = state.current_turn.seat();
            let led_suit = state.current_trick.led_suit();
            let hand = self.hand_mut(seat)?;
            let card = hand
                .iter()
                .copied()
                .find(|card| led_suit.is_none_or(|suit| card.suit == suit))
                .or_else(|| hand.cards().first().copied())
                .ok_or(GameClientError::InvalidAction)?;
            hand.remove(card);
            self.emit_card(state, events, seat, card)?;
            steps += 1;
        }
        Ok(())
    }

    fn emit_card(
        &self,
        state: &mut GameState,
        events: &mut Vec<GameEvent>,
        seat: Seat,
        card: super::Card,
    ) -> Result<(), GameClientError> {
        self.emit(
            state,
            events,
            GameEvent::CardPlayed {
                version: next_version(state)?,
                player_id: local_player_id(seat),
                seat,
                card,
            },
        )?;

        if state.current_trick.is_complete() {
            let winner = state
                .current_trick
                .winner(state.trump)
                .ok_or(GameClientError::InvalidAction)?;
            self.emit(
                state,
                events,
                GameEvent::TrickCompleted {
                    version: next_version(state)?,
                    winner,
                },
            )?;
        } else {
            self.emit(
                state,
                events,
                GameEvent::TurnChanged {
                    version: next_version(state)?,
                    turn: turn_for(seat.next()),
                },
            )?;
        }
        Ok(())
    }

    fn hand_mut(&mut self, seat: Seat) -> Result<&mut Hand, GameClientError> {
        self.hands
            .as_mut()
            .and_then(|hands| hands.get_mut(seat.index()))
            .ok_or(GameClientError::InvalidAction)
    }

    fn emit(
        &self,
        state: &mut GameState,
        events: &mut Vec<GameEvent>,
        event: GameEvent,
    ) -> Result<(), GameClientError> {
        reduce(state, event.clone())
            .map_err(|error| GameClientError::Other(error.to_string()))?;
        events.push(event);
        Ok(())
    }
}

impl GameClient for LocalGameClient {
    fn dispatch(
        &mut self,
        state: &GameState,
        command: GameCommand,
    ) -> Result<Vec<GameEvent>, GameClientError> {
        match command {
            GameCommand::StartGame
                if matches!(
                    state.phase,
                    GamePhase::Lobby | GamePhase::HandComplete
                ) =>
            {
                self.deal(state)
            }
            GameCommand::PlayCard { card } => self.play(state, card),
            GameCommand::SetReady { ready } => {
                Ok(vec![GameEvent::PlayerReady {
                    version: next_version(state)?,
                    player_id: local_player_id(state.my_seat),
                    ready,
                }])
            }
            GameCommand::LeaveGame => Ok(vec![GameEvent::PlayerLeft {
                version: next_version(state)?,
                player_id: local_player_id(state.my_seat),
            }]),
            GameCommand::JoinGame { .. } | GameCommand::SetLocalSeat { .. } => {
                Err(GameClientError::TransportUnavailable)
            }
            GameCommand::PlaceBid { bid } => self.place_bid(state, bid),
            GameCommand::StartGame => Err(GameClientError::InvalidAction),
        }
    }
}

fn next_version(state: &GameState) -> Result<u64, GameClientError> {
    state.version.checked_add(1).ok_or_else(|| {
        GameClientError::Other("game state version overflow".to_string())
    })
}

fn turn_for(seat: Seat) -> super::state::Turn {
    match seat {
        Seat::North => super::state::Turn::North,
        Seat::East => super::state::Turn::East,
        Seat::South => super::state::Turn::South,
        Seat::West => super::state::Turn::West,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::commands::BidStrain;

    fn apply(state: &mut GameState, events: Vec<GameEvent>) {
        for event in events {
            reduce(state, event).unwrap();
        }
    }

    #[test]
    fn local_room_deals_and_runs_a_complete_hand_without_exposing_opponent_hands()
     {
        let mut state = GameState::local_room("LOCAL", "You", Seat::South);
        let mut client = LocalGameClient::new(77);
        let events = client.dispatch(&state, GameCommand::StartGame).unwrap();
        apply(&mut state, events);
        assert_eq!(state.phase, GamePhase::Bidding);
        assert_eq!(state.my_hand.len(), 13);
        assert!(state.players.iter().all(|player| player.card_count == 13));
        assert!(state.players.iter().all(|player| player.ready));

        let events = client
            .dispatch(
                &state,
                GameCommand::PlaceBid {
                    bid: Bid::Contract {
                        level: 1,
                        strain: BidStrain::Hearts,
                    },
                },
            )
            .unwrap();
        apply(&mut state, events);
        assert_eq!(state.phase, GamePhase::Playing);
        assert!(state.is_my_turn());
        assert!(state.my_hand.len() <= 13);
        assert!(state.contract.is_some());

        for _ in 0..13 {
            if state.phase != GamePhase::Playing {
                break;
            }
            let card = state
                .my_hand
                .iter()
                .copied()
                .find(|card| state.can_play_card(*card))
                .expect("local turn has a legal card");
            let events = client
                .dispatch(&state, GameCommand::PlayCard { card })
                .unwrap();
            apply(&mut state, events);
        }

        assert_eq!(state.phase, GamePhase::HandComplete);
        assert_eq!(state.completed_tricks.len(), 13);
        assert!(state.my_hand.is_empty());
        assert!(state.players.iter().all(|player| player.card_count == 0));
    }

    #[test]
    fn local_client_rejects_an_unheld_card_without_changing_projection() {
        let mut state = GameState::local_room("LOCAL", "You", Seat::South);
        let mut client = LocalGameClient::new(9);
        let events = client.dispatch(&state, GameCommand::StartGame).unwrap();
        apply(&mut state, events);
        let events = client
            .dispatch(
                &state,
                GameCommand::PlaceBid {
                    bid: Bid::Contract {
                        level: 1,
                        strain: BidStrain::Clubs,
                    },
                },
            )
            .unwrap();
        apply(&mut state, events);
        let before = state.clone();
        let invalid = Deck::standard()
            .cards()
            .iter()
            .copied()
            .find(|card| !state.my_hand.contains(card))
            .unwrap();
        assert!(matches!(
            client.dispatch(&state, GameCommand::PlayCard { card: invalid }),
            Err(GameClientError::InvalidAction)
        ));
        assert_eq!(state, before);
    }
}
