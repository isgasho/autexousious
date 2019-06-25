use amethyst::ecs::Entity;
use object_model::play::{Grounding, HealthPoints};

use crate::EnergyComponentStorages;

/// Augments an entity with `Energy` components.
#[derive(Debug)]
pub struct EnergyEntityAugmenter;

impl EnergyEntityAugmenter {
    /// Augments an entity with `Energy` components.
    ///
    /// # Parameters
    ///
    /// * `entity`: The entity to augment.
    /// * `energy_component_storages`: Energy specific `Component` storages.
    pub fn augment<'s>(
        entity: Entity,
        EnergyComponentStorages {
            ref mut health_pointses,
            ref mut groundings,
        }: &mut EnergyComponentStorages<'s>,
    ) {
        // Health points.
        health_pointses
            .insert(entity, HealthPoints::default())
            .expect("Failed to insert health_points component.");
        // Grounding.
        groundings
            .insert(entity, Grounding::default())
            .expect("Failed to insert grounding component.");
    }
}

#[cfg(test)]
mod test {
    use amethyst::{
        core::TransformBundle,
        ecs::{Builder, SystemData, World},
        renderer::{types::DefaultBackend, RenderEmptyBundle},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use object_model::play::{Grounding, HealthPoints};

    use super::EnergyEntityAugmenter;
    use crate::EnergyComponentStorages;

    #[test]
    fn augments_entity_with_energy_components() -> Result<(), Error> {
        let assertion = |world: &mut World| {
            let entity = world.create_entity().build();
            {
                let mut energy_component_storages = EnergyComponentStorages::fetch(&world.res);
                EnergyEntityAugmenter::augment(entity, &mut energy_component_storages);
            }

            assert!(world.read_storage::<HealthPoints>().contains(entity));
            assert!(world.read_storage::<Grounding>().contains(entity));
        };

        AmethystApplication::blank()
            .with_bundle(TransformBundle::new())
            .with_bundle(RenderEmptyBundle::<DefaultBackend>::new())
            .with_setup(|world| {
                <EnergyComponentStorages as SystemData>::setup(&mut world.res);
            })
            .with_assertion(assertion)
            .run_isolated()
    }
}