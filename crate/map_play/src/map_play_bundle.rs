use amethyst::{
    core::bundle::{Result, SystemBundle},
    ecs::prelude::*,
};
use typename::TypeName;

use MapAnimationUpdateSystem;

/// Adds the object type update systems to the provided dispatcher.
///
/// The Amethyst `InputBundle` must be added before this bundle.
#[derive(Debug, new)]
pub struct MapPlayBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for MapPlayBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(
            MapAnimationUpdateSystem::new(),
            &MapAnimationUpdateSystem::type_name(),
            // TODO: Pending <https://gitlab.com/azriel91/autexousious/issues/53>
            &[],
        ); // kcov-ignore
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use amethyst_test_support::prelude::*;

    use super::MapPlayBundle;

    #[test]
    fn bundle_build_should_succeed() {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::blank()
                .with_bundle(MapPlayBundle)
                .run()
                .is_ok()
        );
    }
}