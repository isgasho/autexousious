//! User defined configuration types for spawns.

pub use self::{
    spawn::Spawn,
    spawns::{Spawns, SpawnsHandle},
};

mod spawn;
mod spawns;

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use asset_model::config::AssetSlug;
    use kinematic_model::config::{Position, Velocity};
    use serde::{Deserialize, Serialize};
    use toml;

    use super::{Spawn, Spawns};

    const SPAWNS_TOML: &str = r#"
        spawns = [
          { object = "default/fireball" },
          { object = "default/fireball", position = [-35, 47, 10], velocity = [10, -2, 1] },
        ]"#;

    #[derive(Debug, Deserialize, Serialize)]
    struct Config {
        spawns: Spawns,
    }

    #[test]
    fn deserialize_spawns() {
        let config =
            toml::from_str::<Config>(SPAWNS_TOML).expect("Failed to deserialize `Spawns`.");
        let spawns = config.spawns;

        let asset_slug = AssetSlug::from_str("default/fireball")
            .expect("Expected `default/fireball` to be a valid asset slug.");
        assert_eq!(
            Spawns::new(vec![
                Spawn::new(
                    asset_slug.clone(),
                    Position::<i32>::from((0, 0, 0)),
                    Velocity::<i32>::from((0, 0, 0))
                ),
                Spawn::new(
                    asset_slug,
                    Position::<i32>::from((-35, 47, 10)),
                    Velocity::<i32>::from((10, -2, 1))
                )
            ]),
            spawns
        );
    }

}
