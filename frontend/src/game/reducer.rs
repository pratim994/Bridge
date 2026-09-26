use thiserror::Error;

use super::{
    events::GameEvent,
    state::{BidRecord, Contract, GamePhase, GameState, PlayedCard, Seat},
    trick::{CompletedTrick, Trick},
};

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ReducerError {
    #[error("event version {event} is not newer than state version {state}")]
    StaleVersion { event: u64, state: u64 },
    #[error("invalid game-state snapshot: {0}")]
    InvalidSnapshot(String),
    #[error("local card was not present in the player's hand")]
    InvalidLocalCard,
    #[error("trick cannot be completed before all four seats have played")]
    IncompleteTrick,
}

/// Applies one authoritative event. Rejected stale events leave the state unchanged.
pub fn reduce(
    state: &mut GameState,
    event: GameEvent,
) -> Result<(), ReducerError> {
    let version = event_version(&event);
    if let Some(version) = version {
        if version <= state.version {
            return Err(ReducerError::StaleVersion {
                event: version,
                state: state.version,
            });
        }
    }

    match event {
        GameEvent::GameState {
            version,
            state: value,
        } => {
            let mut snapshot: GameState = serde_json::from_value(value)
                .map_err(|error| {
                    ReducerError::InvalidSnapshot(error.to_string())
                })?;
            snapshot.version = version;
            *state = snapshot;
        }
        GameEvent::HandDealt {
            version,
            hand,
            card_counts,
        } => {
            state.my_hand = hand.into();
            for seat in Seat::ALL {
                if let Some(player) = state.player_at_mut(seat) {
                    player.card_count = card_counts[seat.index()] as usize;
                }
            }
            state.phase = GamePhase::Bidding;
            state.completed_tricks.clear();
            state.last_trick_winner = None;
            state.bidding = Default::default();
            state.contract = None;
            state.trump = None;
            state.declarer = None;
            state.tricks_won = 0;
            state.current_turn = super::state::Turn::North;
            state.current_trick = Trick::new(Seat::North);
            state.version = version;
        }
        GameEvent::CardPlayed {
            version,
            player_id,
            seat,
            card,
        } => {
            if seat == state.my_seat {
                if !state.my_hand.remove(card) {
                    return Err(ReducerError::InvalidLocalCard);
                }
            }
            if let Some(player) = state
                .players
                .iter_mut()
                .find(|player| player.id == player_id)
            {
                player.card_count = player.card_count.saturating_sub(1);
            }
            state.current_trick.plays.push(PlayedCard {
                player_id,
                seat,
                card,
            });
            state.version = version;
        }
        GameEvent::PlayerJoined { version, player } => {
            if let Some(existing) =
                state.players.iter_mut().find(|p| p.id == player.id)
            {
                *existing = player;
            } else {
                state.players.push(player);
            }
            state.version = version;
        }
        GameEvent::PlayerLeft { version, player_id } => {
            if let Some(player) = state
                .players
                .iter_mut()
                .find(|player| player.id == player_id)
            {
                player.connected = false;
            }
            state.version = version;
        }
        GameEvent::PlayerReady {
            version,
            player_id,
            ready,
        } => {
            if let Some(player) =
                state.players.iter_mut().find(|p| p.id == player_id)
            {
                player.ready = ready;
            }
            state.version = version;
        }
        GameEvent::BidPlaced {
            version,
            player_id,
            bid,
        } => {
            if let Some((level, strain)) = bid.contract() {
                state.contract = Some(Contract {
                    level,
                    strain,
                    multiplier: 1,
                });
                state.trump = strain.suit();
                state.tricks_required = level + 6;
                state.bidding.consecutive_passes = 0;
                state.declarer = state
                    .players
                    .iter()
                    .find(|player| player.id == player_id)
                    .map(|player| player.seat);
            } else if matches!(bid, super::commands::Bid::Double) {
                if let Some(contract) = &mut state.contract {
                    contract.multiplier = 2;
                }
                state.bidding.consecutive_passes = 0;
            } else if matches!(bid, super::commands::Bid::Redouble) {
                if let Some(contract) = &mut state.contract {
                    contract.multiplier = 4;
                }
                state.bidding.consecutive_passes = 0;
            } else if matches!(bid, super::commands::Bid::Pass) {
                state.bidding.consecutive_passes =
                    state.bidding.consecutive_passes.saturating_add(1);
            } else {
                state.bidding.consecutive_passes = 0;
            }
            state.bidding.bids.push(BidRecord { player_id, bid });
            state.phase = GamePhase::Bidding;
            state.version = version;
        }
        GameEvent::TurnChanged { version, turn } => {
            state.current_turn = turn;
            state.version = version;
        }
        GameEvent::StartTrick { version, leader } => {
            state.current_turn = turn_for_seat(leader);
            state.current_trick = Trick::new(leader);
            state.version = version;
        }
        GameEvent::TrickCompleted { version, winner } => {
            if !state.current_trick.is_complete() {
                return Err(ReducerError::IncompleteTrick);
            }
            if let Some(declarer) = state.declarer {
                if same_partnership(declarer, winner) {
                    state.tricks_won = state.tricks_won.saturating_add(1);
                }
            }
            let completed = CompletedTrick {
                trick: state.current_trick.clone(),
                winner,
            };
            state.completed_tricks.push(completed);
            state.last_trick_winner = Some(winner);
            state.current_trick = Trick::new(winner);
            state.current_turn = turn_for_seat(winner);
            if state.players.iter().all(|player| player.card_count == 0) {
                state.phase = GamePhase::HandComplete;
            }
            state.version = version;
        }
        GameEvent::ScoreUpdated {
            version,
            north_south,
            east_west,
        } => {
            state.score.north_south = north_south;
            state.score.east_west = east_west;
            state.version = version;
        }
        GameEvent::GameStarted { version } => {
            state.phase = GamePhase::Playing;
            if let Some(declarer) = state.declarer {
                let leader = declarer.next();
                state.current_turn = turn_for_seat(leader);
                state.current_trick = Trick::new(leader);
            }
            state.version = version;
        }
        GameEvent::GameCompleted { version } => {
            state.phase = GamePhase::Complete;
            state.version = version;
        }
        GameEvent::GameFinished { version } => {
            state.phase = GamePhase::Complete;
            state.version = version;
        }
        GameEvent::Error { .. } | GameEvent::ChatMessage { .. } => {}
    }
    Ok(())
}

fn event_version(event: &GameEvent) -> Option<u64> {
    match event {
        GameEvent::GameState { version, .. }
        | GameEvent::HandDealt { version, .. }
        | GameEvent::CardPlayed { version, .. }
        | GameEvent::PlayerJoined { version, .. }
        | GameEvent::PlayerLeft { version, .. }
        | GameEvent::PlayerReady { version, .. }
        | GameEvent::BidPlaced { version, .. }
        | GameEvent::TurnChanged { version, .. }
        | GameEvent::StartTrick { version, .. }
        | GameEvent::TrickCompleted { version, .. }
        | GameEvent::ScoreUpdated { version, .. }
        | GameEvent::GameStarted { version }
        | GameEvent::GameCompleted { version }
        | GameEvent::GameFinished { version } => Some(*version),
        GameEvent::Error { .. } | GameEvent::ChatMessage { .. } => None,
    }
}

fn same_partnership(first: Seat, second: Seat) -> bool {
    first.is_same_partnership(second)
}

fn turn_for_seat(seat: Seat) -> super::state::Turn {
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
    use crate::game::{
        Card,
        card::{Rank, Suit},
        commands::{Bid, BidStrain},
    };

    #[test]
    fn card_play_removes_local_card_and_advances_version() {
        let mut state = GameState::demo();
        let card = state.my_hand.cards()[0];
        reduce(
            &mut state,
            GameEvent::CardPlayed {
                version: 1,
                player_id: "south".into(),
                seat: Seat::South,
                card,
            },
        )
        .unwrap();
        assert!(!state.my_hand.contains(&card));
        assert_eq!(state.current_trick.plays.len(), 1);
        assert_eq!(state.players[2].card_count, 12);
        assert_eq!(state.version, 1);
    }

    #[test]
    fn card_play_by_opponent_does_not_expose_or_remove_local_cards() {
        let mut state = GameState::demo();
        let hand = state.my_hand.clone();
        reduce(
            &mut state,
            GameEvent::CardPlayed {
                version: 1,
                player_id: "north".into(),
                seat: Seat::North,
                card: Card {
                    rank: Rank::Two,
                    suit: Suit::Clubs,
                },
            },
        )
        .unwrap();
        assert_eq!(state.my_hand, hand);
        assert_eq!(state.players[0].card_count, 12);
    }

    #[test]
    fn stale_events_are_rejected_without_mutating_state() {
        let mut state = GameState::demo();
        state.version = 2;
        let original = state.clone();
        let result =
            reduce(&mut state, GameEvent::GameCompleted { version: 2 });
        assert_eq!(
            result,
            Err(ReducerError::StaleVersion { event: 2, state: 2 })
        );
        assert_eq!(state, original);
    }

    #[test]
    fn bidding_event_updates_contract_and_bid_history() {
        let mut state = GameState::demo();
        reduce(
            &mut state,
            GameEvent::BidPlaced {
                version: 1,
                player_id: "south".into(),
                bid: Bid::Contract {
                    level: 3,
                    strain: BidStrain::NoTrump,
                },
            },
        )
        .unwrap();
        assert_eq!(
            state.contract.as_ref().map(Contract::label).as_deref(),
            Some("3NT")
        );
        assert_eq!(state.trump, None);
        assert_eq!(state.tricks_required, 9);
        assert_eq!(state.declarer, Some(Seat::South));
        assert_eq!(state.bidding.bids.len(), 1);
        assert_eq!(state.phase, GamePhase::Bidding);
    }

    #[test]
    fn completing_trick_clears_cards_and_counts_declarer_partnership() {
        let mut state = GameState::demo();
        state.current_trick.plays = Seat::ALL
            .into_iter()
            .map(|seat| PlayedCard {
                player_id: seat.short_label().into(),
                seat,
                card: Card {
                    rank: Rank::Two,
                    suit: Suit::Clubs,
                },
            })
            .collect();
        reduce(
            &mut state,
            GameEvent::TrickCompleted {
                version: 1,
                winner: Seat::North,
            },
        )
        .unwrap();
        assert!(state.current_trick.plays.is_empty());
        assert_eq!(state.tricks_won, 4);
    }

    #[test]
    fn score_event_replaces_projection_scores() {
        let mut state = GameState::demo();
        reduce(
            &mut state,
            GameEvent::ScoreUpdated {
                version: 1,
                north_south: 120,
                east_west: 80,
            },
        )
        .unwrap();
        assert_eq!(state.score.north_south, 120);
        assert_eq!(state.score.east_west, 80);
    }
}
