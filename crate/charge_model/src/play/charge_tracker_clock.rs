use amethyst::ecs::{storage::DenseVecStorage, Component};
use derive_deref::{Deref, DerefMut};
use derive_more::From;
use logic_clock::logic_clock;
use serde::{Deserialize, Serialize};
use specs_derive::Component;

/// Logic clock that stores `ChargePoints`.
#[logic_clock]
pub struct ChargeTrackerClock;