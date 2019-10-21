#[cfg(test)]
mod tests {
    use game_model::play::GameEntities;
    use std::collections::HashMap;

    use amethyst::{
        assets::Processor,
        audio::Source,
        core::TransformBundle,
        ecs::{Join, Read, ReadStorage, World, WorldExt},
        renderer::{types::DefaultBackend, RenderEmptyBundle},
        shred::SystemData,
        window::ScreenDimensions,
        Error, State, StateData, Trans,
    };
    use amethyst_test::{
        AmethystApplication, GameUpdate, PopState, HIDPI, SCREEN_HEIGHT, SCREEN_WIDTH,
    };
    use application_event::{AppEvent, AppEventReader};
    use asset_model::{
        config::{AssetSlug, AssetType},
        loaded::{AssetIdMappings, AssetTypeMappings},
    };
    use assets_test::{ASSETS_PATH, MAP_FADE_SLUG};
    use audio_loading::AudioLoadingBundle;
    use character_loading::CharacterLoadingBundle;
    use character_selection_model::CharacterSelections;
    use collision_audio_loading::CollisionAudioLoadingBundle;
    use collision_loading::CollisionLoadingBundle;
    use energy_loading::EnergyLoadingBundle;
    use game_input_model::ControlBindings;
    use game_play_hud::{CpBar, HpBar};
    use kinematic_loading::KinematicLoadingBundle;
    use kinematic_model::config::Position;
    use loading::{LoadingBundle, LoadingState};
    use loading_model::loaded::{AssetLoadStage, LoadStage};
    use map_loading::MapLoadingBundle;
    use map_selection::MapSelectionStatus;
    use map_selection_model::MapSelection;
    use object_type::ObjectType;
    use sequence_loading::SequenceLoadingBundle;
    use spawn_loading::SpawnLoadingBundle;
    use sprite_loading::SpriteLoadingBundle;
    use typename::TypeName;
    use ui_audio_loading::UiAudioLoadingBundle;

    use game_loading::{
        CharacterAugmentRectifySystem, CharacterAugmentRectifySystemData, CharacterAugmentStatus,
        CharacterSelectionSpawningSystem, GameLoadingStatus,
    };

    #[test]
    fn returns_if_augment_status_is_not_rectify() -> Result<(), Error> {
        run_test(
            |world| {
                let mut game_loading_status = GameLoadingStatus::new();
                game_loading_status.character_augment_status = CharacterAugmentStatus::Prefab;
                world.insert(game_loading_status);
            },
            |world| {
                let char_entity = world
                    .read_resource::<GameEntities>()
                    .objects
                    .get(&ObjectType::Character)
                    .expect("Expected `Character` entities to exist.")
                    .iter()
                    .next()
                    .copied()
                    .expect("Expected character entity to exist.");
                assert_eq!(
                    // Default is inserted by character augmenter.
                    Some(Position::<f32>::new(0., 0., 0.)).as_ref(),
                    world.read_storage::<Position<f32>>().get(char_entity)
                );
            },
        )
    }

    #[test]
    fn updates_position_to_middle_of_map() -> Result<(), Error> {
        run_test(
            |world| {
                let mut game_loading_status = GameLoadingStatus::new();
                game_loading_status.character_augment_status = CharacterAugmentStatus::Rectify;
                world.insert(game_loading_status);
            },
            |world| {
                let char_entity = world
                    .read_resource::<GameEntities>()
                    .objects
                    .get(&ObjectType::Character)
                    .expect("Expected `Character` entities to exist.")
                    .iter()
                    .next()
                    .copied()
                    .expect("Expected character entity to exist.");
                // kcov-ignore-start
                assert_eq!(
                    // kcov-ignore-end
                    // See assets_test/assets/test/map/fade/map.yaml
                    Position::<f32>::new(400., 200., 100.),
                    *world
                        .read_storage::<Position<f32>>()
                        .get(char_entity)
                        .expect("Expected entity to have position.")
                );
                assert_eq!(
                    CharacterAugmentStatus::Complete,
                    world
                        .read_resource::<GameLoadingStatus>()
                        .character_augment_status
                );
            },
        )
    }

    #[test]
    fn creates_hp_and_cp_bar_entities_per_character_selection() -> Result<(), Error> {
        run_test(
            |world| {
                let mut game_loading_status = GameLoadingStatus::new();
                game_loading_status.character_augment_status = CharacterAugmentStatus::Rectify;
                world.insert(game_loading_status);
            },
            |world| {
                let (hp_bars, cp_bars) =
                    world.system_data::<(ReadStorage<'_, HpBar>, ReadStorage<'_, CpBar>)>();
                assert_eq!(1, (&hp_bars).join().count());
                assert_eq!(1, (&cp_bars).join().count());
            },
        )
    }

    fn run_test<FnS, FnA>(fn_setup: FnS, fn_assert: FnA) -> Result<(), Error>
    where
        FnS: Fn(&mut World) + Send + Sync + 'static,
        FnA: Fn(&mut World) + Send + Sync + 'static,
    {
        let wait_for_load = WaitForLoad {
            slug: MAP_FADE_SLUG.clone(),
        };

        AmethystApplication::blank()
            .with_custom_event_type::<AppEvent, AppEventReader>()
            .with_bundle(TransformBundle::new())
            .with_bundle(RenderEmptyBundle::<DefaultBackend>::new())
            .with_resource(ScreenDimensions::new(SCREEN_WIDTH, SCREEN_HEIGHT, HIDPI))
            .with_ui_bundles::<ControlBindings>()
            .with_system(Processor::<Source>::new(), "source_processor", &[])
            .with_bundle(SpriteLoadingBundle::new())
            .with_bundle(SequenceLoadingBundle::new())
            .with_bundle(AudioLoadingBundle::new())
            .with_bundle(KinematicLoadingBundle::new())
            .with_bundle(LoadingBundle::new(ASSETS_PATH.clone()))
            .with_bundle(CollisionLoadingBundle::new())
            .with_bundle(SpawnLoadingBundle::new())
            .with_bundle(MapLoadingBundle::new())
            .with_bundle(CharacterLoadingBundle::new())
            .with_bundle(EnergyLoadingBundle::new())
            .with_bundle(CollisionAudioLoadingBundle::new(ASSETS_PATH.clone()))
            .with_bundle(UiAudioLoadingBundle::new(ASSETS_PATH.clone()))
            .with_effect(|world| CharacterAugmentRectifySystemData::setup(world))
            .with_state(|| LoadingState::new(PopState))
            .with_state(|| wait_for_load)
            .with_effect(|world| setup_map_selection(world, &*MAP_FADE_SLUG))
            .with_effect(|world| {
                let mut game_loading_status = GameLoadingStatus::new();
                game_loading_status.character_augment_status = CharacterAugmentStatus::Prefab;
                world.insert(game_loading_status);

                let asset_id = {
                    let asset_type_mappings = world.read_resource::<AssetTypeMappings>();
                    asset_type_mappings
                        .iter_ids(&AssetType::Object(ObjectType::Character))
                        .next()
                        .copied()
                        .expect("Expected at least one character to be loaded.")
                };
                let mut character_selections = HashMap::new();
                character_selections.insert(123, asset_id);
                let character_selections = CharacterSelections::new(character_selections);
                world.insert(character_selections);
            })
            .with_system_single(CharacterSelectionSpawningSystem::new(), "", &[])
            .with_effect(fn_setup)
            .with_system_single(
                CharacterAugmentRectifySystem,
                CharacterAugmentRectifySystem::type_name(),
                &[],
            ) // kcov-ignore
            .with_assertion(fn_assert)
            .run()
    }

    fn setup_map_selection(world: &mut World, slug: &AssetSlug) {
        let map_asset_id = world
            .read_resource::<AssetIdMappings>()
            .id(slug)
            .copied()
            .unwrap_or_else(|| panic!("Expected `{}` to be loaded.", slug));

        world.insert(MapSelection::Id(map_asset_id));
        world.insert(MapSelectionStatus::Confirmed);
    }

    #[derive(Debug)]
    struct WaitForLoad {
        slug: AssetSlug,
    }
    impl<T, E> State<T, E> for WaitForLoad
    where
        T: GameUpdate,
        E: Send + Sync + 'static,
    {
        fn update(&mut self, data: StateData<'_, T>) -> Trans<T, E> {
            data.data.update(&data.world);

            let (asset_id_mappings, asset_load_stage) = data
                .world
                .system_data::<(Read<'_, AssetIdMappings>, Read<'_, AssetLoadStage>)>();
            if let Some(LoadStage::Complete) = asset_id_mappings
                .id(&self.slug)
                .and_then(|asset_id| asset_load_stage.get(*asset_id))
            {
                Trans::Pop
            } else {
                Trans::None
            }
        }
    }
}
