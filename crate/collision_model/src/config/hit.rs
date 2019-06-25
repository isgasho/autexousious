use derive_new::new;
use object_status_model::config::StunPoints;
use serde::{Deserialize, Serialize};

use crate::config::{HitLimit, HitRepeatDelay};

/// Configuration of a hit interaction.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq, Hash, Serialize, new)]
#[serde(default, deny_unknown_fields)]
pub struct Hit {
    /// Number of ticks to wait before another hit may occur.
    pub repeat_delay: HitRepeatDelay,
    /// Number of objects a `Hit` may collide with.
    pub hit_limit: HitLimit,
    /// Amount of health points (HP) to subtract on collision.
    pub hp_damage: u32,
    /// Amount of skill points (SP) to subtract on collision.
    pub sp_damage: u32,
    /// Amount of stun points to inflict on collision.
    pub stun: StunPoints,
}

#[cfg(test)]
mod test {
    use object_status_model::config::StunPoints;
    use toml;

    use super::Hit;
    use crate::config::{HitLimit, HitRepeatDelay};

    const HIT_TOML: &str = r#"
        repeat_delay = 1
        hit_limit = 2
        hp_damage = 3
        sp_damage = 4
        stun = 5
    "#;

    #[test]
    fn deserialize_hit() {
        let hit_deserialized =
            toml::from_str::<Hit>(HIT_TOML).expect("Failed to deserialize `Hit`.");

        let expected = Hit::new(
            HitRepeatDelay::new(1),
            HitLimit::Limit(2),
            3,
            4,
            StunPoints::new(5),
        );

        assert_eq!(expected, hit_deserialized);
    }
}