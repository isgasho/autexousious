#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Provides a wrapper `State` around asset loading.

pub use crate::{asset_loader::AssetLoader, loading_state::LoadingState};

mod asset_loader;
mod loading_state;
