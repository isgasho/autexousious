use amethyst::{core::transform::Transform, ecs::prelude::*};
use object_model::entity::Kinematics;

/// Updates each entity's `Transform` based on their `Position` in game.
///
/// This system should be run after all other systems that affect kinematics have run.
#[derive(Debug, Default, new)]
pub(crate) struct ObjectTransformUpdateSystem;

type ObjectTransformUpdateSystemData<'s> = (
    ReadStorage<'s, Kinematics<f32>>,
    WriteStorage<'s, Transform>,
);

impl<'s> System<'s> for ObjectTransformUpdateSystem {
    type SystemData = ObjectTransformUpdateSystemData<'s>;

    fn run(&mut self, (kinematics_storage, mut transform_storage): Self::SystemData) {
        for (kinematics, mut transform) in (&kinematics_storage, &mut transform_storage).join() {
            let position = &kinematics.position;
            transform.translation[0] = position.x;
            transform.translation[1] = position.y + position.z;
        }
    }
}

#[cfg(test)]
mod test {
    use amethyst::{
        core::{cgmath::Vector3, transform::Transform},
        ecs::prelude::*,
    };
    use amethyst_test_support::*;
    use object_model::entity::{Kinematics, Position, Velocity};

    use super::ObjectTransformUpdateSystem;

    #[test]
    fn updates_transform_with_x_and_yz() {
        let setup = |world: &mut World| {
            // Create entity with Kinematics
            let position = Position::<f32>::new(-2., -2., -2.);
            let velocity = Velocity::default();

            let mut transform = Transform::default();
            transform.translation = Vector3::new(10., 20., 0.);

            let entity = world
                .create_entity()
                .with(Kinematics::new(position, velocity))
                .with(transform)
                .build();

            world.add_resource(EffectReturn(entity));
        };

        let assertion = |world: &mut World| {
            let entity = world.read_resource::<EffectReturn<Entity>>().0;
            let store = world.read_storage::<Transform>();

            let mut transform = Transform::default();
            transform.translation = Vector3::new(-2., -4., 0.);

            assert_eq!(Some(&transform), store.get(entity));
        };

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::base()
                .with_system(
                    ObjectTransformUpdateSystem::new(),
                    "object_transform_update_system",
                    &[]
                )
                .with_setup(setup)
                .with_assertion(assertion)
                .run()
                .is_ok()
        );
    }
}