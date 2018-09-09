use std::path::Path;

use amethyst::{assets::Loader, prelude::*, renderer::MaterialTextureSet};
use application::{load_in, ErrorKind, Format, Result};
use map_model::{
    config::MapDefinition,
    loaded::{Map, MapHandle, Margins},
};
use sprite_loading::{SpriteLoader, SpriteRenderAnimationLoader};

/// Loads assets specified by map configuration into the loaded map model.
#[derive(Debug)]
pub struct MapLoader;

impl MapLoader {
    /// Returns the loaded `Map` referenced by the asset record.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to store the map's assets.
    /// * `base_dir`: Base directory from which to load the map.
    pub fn load(world: &World, base_dir: &Path) -> Result<MapHandle> {
        debug!("Loading map in `{}`", base_dir.display());

        let map_definition = load_in::<MapDefinition, _>(base_dir, "map.toml", Format::Toml, None)?;
        let texture_index_offset = world.read_resource::<MaterialTextureSet>().len() as u64;

        let sprite_load_result = SpriteLoader::load(world, texture_index_offset, base_dir);
        let loaded_sprites = match sprite_load_result {
            Ok(loaded_sprites) => Ok(Some(loaded_sprites)),
            Err(e) => match e.kind() {
                ErrorKind::Find(..) => Ok(None),
                _ => Err(e),
            },
        }?;

        let (sprite_sheet_handles, animation_handles) = {
            if let Some((sprite_sheet_handles, _texture_handles)) = loaded_sprites {
                let animation_handles = SpriteRenderAnimationLoader::load_into_vec(
                    world,
                    map_definition.layers.iter(),
                    texture_index_offset,
                );
                (Some(sprite_sheet_handles), Some(animation_handles))
            } else {
                (None, None)
            }
        };

        let margins = Margins::from(map_definition.header.bounds);

        let map = Map::new(
            map_definition,
            margins,
            sprite_sheet_handles,
            animation_handles,
        );

        let loader = world.read_resource::<Loader>();
        let map_handle = loader.load_from_data(map, (), &world.read_resource());
        Ok(map_handle)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use amethyst::assets::AssetStorage;
    use amethyst_test_support::prelude::*;
    use application::resource::dir::assets_dir;
    use map_model::loaded::{Map, MapHandle};

    use super::MapLoader;
    use MapLoadingBundle;

    // Successful case covered by `MapLoadingBundle` test

    #[test]
    fn loads_map_without_sprites() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::render_base("loads_map_without_sprites", false)
                .with_bundle(MapLoadingBundle)
                .with_effect(|world| {
                    let mut map_path = assets_dir(Some(development_base_dirs!()))
                        .expect("Expected to find `assets` directory in crate root.");
                    map_path.extend(Path::new("test/map/empty").iter());

                    let map_handle =
                        MapLoader::load(world, &map_path).expect("Failed to load map.");

                    world.add_resource(EffectReturn(map_handle));
                }).with_assertion(|world| {
                    let map_handle = world.read_resource::<EffectReturn<MapHandle>>().0.clone();
                    let map_store = world.read_resource::<AssetStorage<Map>>();
                    let map = map_store
                        .get(&map_handle)
                        .expect("Expected map to be loaded.");

                    // See empty/map.toml
                    assert!(map.animation_handles.is_none());
                }).run()
                .is_ok()
        );
    }
}