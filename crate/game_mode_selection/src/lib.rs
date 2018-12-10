#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! State where game mode selection takes place.

pub use crate::game_mode_selection_state::{
    GameModeSelectionState, GameModeSelectionStateBuilder, GameModeSelectionStateDelegate,
};
pub(crate) use crate::game_mode_selection_trans::GameModeSelectionTrans;

mod game_mode_selection_state;
mod game_mode_selection_trans;
