use collision_model::config::{Body, Interactions};
use derive_new::new;
use sequence_model::config::Wait;
use serde::{Deserialize, Serialize};
use spawn_model::config::Spawns;
use sprite_model::config::SpriteRef;

/// Common object behaviour specification that can change each tick.
///
/// Frames are the level of detail that should carry the following information:
///
/// * **Render information:** Sprite sheet, sprite, and transition delay.
/// * **Interaction:** Collision zones, type of interactions.
/// * **Effects:** Sound(s) to play.
/// * **Spawning:** Spawning additional object(s).
/// * **Weapon:** Where an active weapon should be.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, new)]
#[serde(default)]
pub struct ObjectFrame {
    /// Number of ticks to wait before the sequence switches to the next frame.
    pub wait: Wait,
    /// Sprite to render.
    pub sprite: SpriteRef,
    /// Hittable volume of the object.
    pub body: Body,
    /// Interaction volumes of the object.
    pub interactions: Interactions,
    /// Objects to spawn.
    pub spawns: Spawns,
}