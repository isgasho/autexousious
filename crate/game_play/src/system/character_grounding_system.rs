use amethyst::{assets::AssetStorage, ecs::prelude::*};
use map_model::loaded::Map;
use map_selection::MapSelection;
use object_model::entity::{CharacterStatus, Grounding, Kinematics};

/// Updates `Character` kinematics based on sequence.
#[derive(Debug, Default, TypeName, new)]
pub(crate) struct CharacterGroundingSystem;

type CharacterGroundingSystemData<'s> = (
    Read<'s, MapSelection>,
    Read<'s, AssetStorage<Map>>,
    WriteStorage<'s, Kinematics<f32>>,
    WriteStorage<'s, CharacterStatus>,
);

impl<'s> System<'s> for CharacterGroundingSystem {
    type SystemData = CharacterGroundingSystemData<'s>;

    fn run(
        &mut self,
        (map_selection, maps, mut kinematics_storage, mut status_storage): Self::SystemData,
    ) {
        let map_handle = map_selection.map_handle.as_ref();
        if map_handle.is_none() {
            // Game is not running.
            // TODO: Use custom `GameData` / state specific dispatcher
            return;
        }

        let map_handle = map_handle.unwrap().clone();
        let map_margins = {
            maps.get(&map_handle)
                .map(|map| map.margins)
                .expect("Expected map to be loaded.")
        };

        for (mut kinematics, mut status) in (&mut kinematics_storage, &mut status_storage).join() {
            if kinematics.position[1] > map_margins.bottom {
                kinematics.velocity[1] += -1.7;
                status.object_status.grounding = Grounding::Airborne;
            } else {
                kinematics.position[1] = map_margins.bottom;
                kinematics.velocity[1] = 0.;
                status.object_status.grounding = Grounding::OnGround;
            }
        }
    }
}
