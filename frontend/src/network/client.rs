use thiserror::Error;

use crate::game::{commands::GameCommand, events::GameEvent, state::GameState};

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum GameClientError {
    #[error("action is not available in the current game state")]
    InvalidAction,
    #[error("this command requires a multiplayer transport")]
    TransportUnavailable,
    #[error("game client failed: {0}")]
    Other(String),
}

/// UI-facing command boundary. Transport adapters return events; they never mutate UI state.
pub trait GameClient: Send {
    fn dispatch(
        &mut self,
        state: &GameState,
        command: GameCommand,
    ) -> Result<Vec<GameEvent>, GameClientError>;
}
