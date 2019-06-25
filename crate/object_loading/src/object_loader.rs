use std::collections::HashMap;

use amethyst::{assets::Handle, renderer::SpriteRender, Error};
use collision_model::{
    config::{Body, Interactions},
    loaded::{BodySequence, BodySequenceHandle, InteractionsSequence, InteractionsSequenceHandle},
};
use fnv::FnvHashMap;
use object_model::{
    config::{GameObjectFrame, GameObjectSequence, ObjectDefinition},
    loaded::{GameObject, Object, ObjectWrapper},
};
use sequence_model::{
    config::Wait,
    loaded::{SequenceEndTransition, WaitSequence, WaitSequenceHandle},
};
use serde::{Deserialize, Serialize};
use spawn_model::{
    config::Spawns,
    loaded::{SpawnsSequence, SpawnsSequenceHandle},
};
use sprite_model::loaded::{SpriteRenderSequence, SpriteRenderSequenceHandle};

use crate::ObjectLoaderParams;

/// Loads assets specified by object configuration into the loaded object model.
#[derive(Debug)]
pub struct ObjectLoader;

impl ObjectLoader {
    /// Returns the loaded `Object` referenced by the asset record.
    ///
    /// # Parameters
    ///
    /// * `object_loader_params`: Entry of the object's configuration.
    /// * `object_definition`: Object definition configuration.
    pub fn load<O>(
        ObjectLoaderParams {
            loader,
            wait_sequence_assets,
            sprite_render_sequence_assets,
            body_sequence_assets,
            interactions_sequence_assets,
            spawns_sequence_assets,
            sprite_sheet_handles,
            body_assets,
            interactions_assets,
            spawns_assets,
        }: ObjectLoaderParams,
        object_definition: &ObjectDefinition<O::GameObjectSequence>,
    ) -> Result<O::ObjectWrapper, Error>
    where
        O: GameObject,
        <O as GameObject>::SequenceId: for<'de> Deserialize<'de> + Serialize,
    {
        let sequence_end_transitions = object_definition
            .sequences
            .iter()
            .map(|(sequence_id, sequence)| {
                (
                    *sequence_id,
                    SequenceEndTransition::new(sequence.object_sequence().next),
                )
            })
            .collect::<FnvHashMap<_, _>>();

        // Load component sequences
        let sequences_handles = (
            HashMap::<O::SequenceId, WaitSequenceHandle>::new(),
            HashMap::<O::SequenceId, SpriteRenderSequenceHandle>::new(),
            HashMap::<O::SequenceId, BodySequenceHandle>::new(),
            HashMap::<O::SequenceId, InteractionsSequenceHandle>::new(),
            HashMap::<O::SequenceId, SpawnsSequenceHandle>::new(),
        );
        let (
            wait_sequence_handles,
            sprite_render_sequence_handles,
            body_sequence_handles,
            interactions_sequence_handles,
            spawns_sequence_handles,
        ) = object_definition.sequences.iter().fold(
            sequences_handles,
            |(
                mut wait_sequence_handles,
                mut sprite_render_sequence_handles,
                mut body_sequence_handles,
                mut interactions_sequence_handles,
                mut spawns_sequence_handles,
            ),
             (sequence_id, sequence)| {
                let wait_sequence = WaitSequence::new(
                    sequence
                        .object_sequence()
                        .frames
                        .iter()
                        .map(|frame| frame.object_frame().wait)
                        .collect::<Vec<Wait>>(),
                );
                let sprite_render_sequence = SpriteRenderSequence::new(
                    sequence
                        .object_sequence()
                        .frames
                        .iter()
                        .map(|frame| {
                            let sprite_ref = &frame.object_frame().sprite;
                            let sprite_sheet = sprite_sheet_handles[sprite_ref.sheet].clone();
                            let sprite_number = sprite_ref.index;
                            SpriteRender {
                                sprite_sheet,
                                sprite_number,
                            }
                        })
                        .collect::<Vec<SpriteRender>>(),
                );
                let body_sequence = BodySequence::new(
                    sequence
                        .object_sequence()
                        .frames
                        .iter()
                        .map(|frame| {
                            loader.load_from_data(
                                frame.object_frame().body.clone(),
                                (),
                                body_assets,
                            )
                        })
                        .collect::<Vec<Handle<Body>>>(),
                );
                let interactions_sequence = InteractionsSequence::new(
                    sequence
                        .object_sequence()
                        .frames
                        .iter()
                        .map(|frame| {
                            loader.load_from_data(
                                frame.object_frame().interactions.clone(),
                                (),
                                interactions_assets,
                            )
                        })
                        .collect::<Vec<Handle<Interactions>>>(),
                );
                let spawns_sequence = SpawnsSequence::new(
                    sequence
                        .object_sequence()
                        .frames
                        .iter()
                        .map(|frame| {
                            loader.load_from_data(
                                frame.object_frame().spawns.clone(),
                                (),
                                spawns_assets,
                            )
                        })
                        .collect::<Vec<Handle<Spawns>>>(),
                );

                let wait_sequence_handle =
                    loader.load_from_data(wait_sequence, (), wait_sequence_assets);
                let sprite_render_sequence_handle = loader.load_from_data(
                    sprite_render_sequence,
                    (),
                    sprite_render_sequence_assets,
                );
                let body_sequence_handle =
                    loader.load_from_data(body_sequence, (), body_sequence_assets);
                let interactions_sequence_handle =
                    loader.load_from_data(interactions_sequence, (), interactions_sequence_assets);
                let spawns_sequence_handle =
                    loader.load_from_data(spawns_sequence, (), spawns_sequence_assets);

                let sequence_id = *sequence_id;

                wait_sequence_handles.insert(sequence_id, wait_sequence_handle);
                sprite_render_sequence_handles.insert(sequence_id, sprite_render_sequence_handle);
                body_sequence_handles.insert(sequence_id, body_sequence_handle);
                interactions_sequence_handles.insert(sequence_id, interactions_sequence_handle);
                spawns_sequence_handles.insert(sequence_id, spawns_sequence_handle);

                (
                    wait_sequence_handles,
                    sprite_render_sequence_handles,
                    body_sequence_handles,
                    interactions_sequence_handles,
                    spawns_sequence_handles,
                )
            },
        );

        let object = Object::new(
            wait_sequence_handles,
            sprite_render_sequence_handles,
            body_sequence_handles,
            interactions_sequence_handles,
            spawns_sequence_handles,
            sequence_end_transitions.into(),
        );
        let wrapper = O::ObjectWrapper::new(object);

        Ok(wrapper)
    }
}

#[cfg(test)]
mod test {
    use amethyst::{
        assets::{AssetStorage, Processor, ProgressCounter},
        core::TransformBundle,
        ecs::Read,
        renderer::{types::DefaultBackend, RenderEmptyBundle, SpriteSheet, Texture},
    };
    use amethyst_test::AmethystApplication;
    use application::{load_in, Format};
    use asset_model::config::AssetRecord;
    use assets_test::{CHAR_BAT_PATH, CHAR_BAT_SLUG};
    use character_model::{
        config::CharacterDefinition,
        loaded::{Character, CharacterObjectWrapper},
    };
    use collision_loading::CollisionLoadingBundle;
    use sequence_loading::SequenceLoadingBundle;
    use spawn_loading::SpawnLoadingBundle;
    use sprite_loading::SpriteLoader;
    use sprite_model::config::SpritesDefinition;
    use typename::TypeName;

    use super::ObjectLoader;
    use crate::{ObjectDefinitionToWrapperProcessor, ObjectLoaderParams, ObjectLoaderSystemData};

    #[test]
    fn loads_object_assets() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::blank()
                .with_bundle(TransformBundle::new())
                .with_bundle(RenderEmptyBundle::<DefaultBackend>::new())
                .with_bundle(CollisionLoadingBundle::new())
                .with_bundle(SpawnLoadingBundle::new())
                .with_bundle(SequenceLoadingBundle::new())
                .with_system(
                    ObjectDefinitionToWrapperProcessor::<Character>::new(),
                    ObjectDefinitionToWrapperProcessor::<Character>::type_name(),
                    &[]
                )
                .with_system(Processor::<Character>::new(), "character_processor", &[])
                .with_effect(|world| {
                    let asset_record =
                        AssetRecord::new(CHAR_BAT_SLUG.clone(), CHAR_BAT_PATH.clone());

                    let character_definition = load_in::<CharacterDefinition, _>(
                        &asset_record.path,
                        "object.toml",
                        Format::Toml,
                        None,
                    )
                    .expect("Failed to load object.toml into CharacterDefinition");

                    let object_wrapper = {
                        let sprites_definition = load_in::<SpritesDefinition, _>(
                            &asset_record.path,
                            "sprites.toml",
                            Format::Toml,
                            None,
                        )
                        .expect("Failed to load sprites_definition.");

                        let (
                            ObjectLoaderSystemData {
                                loader,
                                wait_sequence_assets,
                                sprite_render_sequence_assets,
                                body_sequence_assets,
                                interactions_sequence_assets,
                                spawns_sequence_assets,
                                body_assets,
                                interactions_assets,
                                spawns_assets,
                            },
                            texture_assets,
                            sprite_sheet_assets,
                        ) = world.system_data::<TestSystemData>();

                        // TODO: <https://gitlab.com/azriel91/autexousious/issues/94>
                        let sprite_sheet_handles = SpriteLoader::load(
                            &mut ProgressCounter::default(),
                            &loader,
                            &texture_assets,
                            &sprite_sheet_assets,
                            &sprites_definition,
                            &asset_record.path,
                        )
                        .expect("Failed to load sprites.");
                        let sprite_sheet_handles = &sprite_sheet_handles;

                        ObjectLoader::load::<Character>(
                            ObjectLoaderParams {
                                loader: &loader,
                                wait_sequence_assets: &wait_sequence_assets,
                                sprite_render_sequence_assets: &sprite_render_sequence_assets,
                                body_sequence_assets: &body_sequence_assets,
                                interactions_sequence_assets: &interactions_sequence_assets,
                                spawns_sequence_assets: &spawns_sequence_assets,
                                sprite_sheet_handles,
                                body_assets: &body_assets,
                                interactions_assets: &interactions_assets,
                                spawns_assets: &spawns_assets,
                            },
                            &character_definition.object_definition,
                        )
                        .expect("Failed to load object")
                    };

                    world.add_resource(object_wrapper);
                })
                .with_assertion(|world| {
                    let object_wrapper = world.read_resource::<CharacterObjectWrapper>();

                    macro_rules! assert_component_sequence_count {
                        ($component_sequence_field:ident) => {
                            assert_eq!(
                                28,
                                object_wrapper.$component_sequence_field.len(),
                                concat!(
                                    "Expected 28 ",
                                    stringify!($component_sequence_field),
                                    " to be loaded.",
                                    "Check `bat/object.toml` for number of sequences."
                                )
                            );
                        };
                    }

                    assert_component_sequence_count!(wait_sequence_handles);
                    assert_component_sequence_count!(sprite_render_sequence_handles);
                    assert_component_sequence_count!(body_sequence_handles);
                    assert_component_sequence_count!(interactions_sequence_handles);
                })
                .run_isolated()
                .is_ok()
        );
    }

    type TestSystemData<'s> = (
        ObjectLoaderSystemData<'s>,
        Read<'s, AssetStorage<Texture>>,
        Read<'s, AssetStorage<SpriteSheet>>,
    );
}