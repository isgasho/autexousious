use amethyst::{
    core::bundle::{Result, SystemBundle},
    ecs::prelude::*,
};
use application_event::AppEventVariant;
use stdio_spi::MapperSystem;
use typename::TypeName;

use GameModeSelectionEventStdinMapper;

/// Adds a `MapperSystem<GameModeSelectionEventStdinMapper>` to the `World`.
#[derive(Debug, new)]
pub struct GameModeSelectionStdioBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GameModeSelectionStdioBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(
            MapperSystem::<GameModeSelectionEventStdinMapper>::new(
                AppEventVariant::GameModeSelection,
            ),
            &MapperSystem::<GameModeSelectionEventStdinMapper>::type_name(),
            &[],
        ); // kcov-ignore
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use amethyst::shrev::EventChannel;
    use amethyst_test_support::prelude::*;
    use stdio_spi::VariantAndTokens;

    use super::GameModeSelectionStdioBundle;

    #[test]
    fn bundle_should_add_mapper_system_to_dispatcher() {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::blank()
                .with_bundle(GameModeSelectionStdioBundle::new())
                // kcov-ignore-start
                .with_effect(|world| {
                    world.read_resource::<EventChannel<VariantAndTokens>>();
                })
                // kcov-ignore-end
                .run()
                .is_ok()
        );
    }
}