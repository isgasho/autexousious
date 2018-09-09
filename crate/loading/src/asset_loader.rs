use std::collections::HashMap;
use std::path::Path;

use amethyst::{
    assets::{Loader, Progress},
    prelude::*,
    renderer::ScreenDimensions,
};
use asset_loading::AssetDiscovery;
use game_model::{
    config::{AssetRecord, AssetSlugBuilder},
    loaded::{CharacterAssets, MapAssets},
};
use map_loading::MapLoader;
use map_model::{
    config::{MapBounds, MapDefinition, MapHeader},
    loaded::{Map, MapHandle, Margins},
};
use object_loading::CharacterLoader;
use object_model::ObjectType;
use strum::IntoEnumIterator;

/// Provides functions to load game assets.
#[derive(Debug)]
pub struct AssetLoader;

impl AssetLoader {
    /// Loads game assets into the `World` from the specified assets directory.
    ///
    /// When this function returns, the `World` will be populated with the `CharacterAssets` and
    /// `MapAssets` resources.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to load the game assets into.
    /// * `assets_dir`: Base directory containing all assets to load.
    /// * `progress`: Tracker for loading progress.
    pub fn load<P>(world: &mut World, assets_dir: &Path, progress: P)
    where
        P: Progress,
    {
        let asset_index = AssetDiscovery::asset_index(assets_dir);

        debug!("Indexed assets: {:?}", &asset_index);

        Self::load_objects(world, asset_index.objects);
        Self::load_maps(world, progress, asset_index.maps);
    }

    /// Loads object configuration into the `World` from the specified assets directory.
    ///
    /// When this function returns, the `World` will be populated with the `CharacterAssets`
    /// resource.
    ///
    /// The normal use case for `AssetLoader` is to use the `load` function which loads both objects
    /// and maps. This method is exposed for testing the loading itself.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to load the object assets into.
    /// * `indexed_objects`: Index of object assets.
    pub fn load_objects(
        world: &mut World,
        mut indexed_objects: HashMap<ObjectType, Vec<AssetRecord>>,
    ) {
        ObjectType::iter()
            .filter_map(|object_type| indexed_objects.remove_entry(&object_type))
            .for_each(|(object_type, asset_records)| {
                // asset_records is the list of records for one object type
                match object_type {
                    ObjectType::Character => {
                        let character_assets = asset_records
                            .into_iter()
                            .filter_map(|asset_record| {
                                debug!(
                                    "Loading `{}` from: `{}`",
                                    asset_record.asset_slug,
                                    asset_record.path.display()
                                );

                                let load_result = CharacterLoader::load(world, &asset_record);

                                if let Err(e) = load_result {
                                    error!("Failed to load character. Reason: \n\n```\n{}\n```", e);

                                    None
                                } else {
                                    Some((asset_record.asset_slug, load_result.unwrap()))
                                }
                            }).collect::<CharacterAssets>();

                        debug!("Loaded character assets: `{:?}`", character_assets);

                        world.add_resource(character_assets);
                    }
                };
            });
    }

    /// Loads map configuration into the `World` from the specified assets directory.
    ///
    /// When this function returns, the `World` will be populated with the `MapAssets` resource.
    ///
    /// The normal use case for `AssetLoader` is to use the `load` function which loads both objects
    /// and maps. This method is exposed for testing the loading itself.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to load the map assets into.
    /// * `progress`: Tracker for loading progress.
    /// * `indexed_maps`: Index of map assets.
    pub fn load_maps<P>(world: &mut World, progress: P, indexed_maps: Vec<AssetRecord>)
    where
        P: Progress,
    {
        let mut map_assets = indexed_maps
            .into_iter()
            .filter_map(|asset_record| {
                let load_result = MapLoader::load(world, &asset_record.path);

                if let Err(e) = load_result {
                    error!("Failed to load map. Reason: \n\n```\n{}\n```", e);

                    None
                } else {
                    Some((asset_record.asset_slug, load_result.unwrap()))
                }
            }).collect::<MapAssets>();

        // Default Blank Map
        let (width, height) = {
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };

        let depth = 200;
        let bounds = MapBounds::new(0, 0, 0, width as u32, height as u32 - depth, depth);
        let header = MapHeader::new("Blank Screen".to_string(), bounds);
        let layers = Vec::new();
        let definition = MapDefinition::new(header, layers);
        let margins = Margins::from(definition.header.bounds);
        let map = Map::new(definition, margins, None, None);

        let map_handle: MapHandle = {
            let loader = world.read_resource::<Loader>();
            loader.load_from_data(map, progress, &world.read_resource())
        };
        let asset_slug = AssetSlugBuilder::default()
            .namespace("built-in".to_string())
            .name("blank".to_string())
            .build()
            .expect("Expected hard coded asset slug to be valid.");

        map_assets.insert(asset_slug, map_handle);

        debug!("Loaded map assets: `{:?}`", map_assets);

        world.add_resource(map_assets);
    }
}