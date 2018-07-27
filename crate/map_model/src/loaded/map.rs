use amethyst::{
    animation::Animation,
    assets::{Asset, Error, Handle, ProcessingState},
    ecs::prelude::*,
    renderer::Material,
};
use sprite_model::loaded::SpriteMaterialMesh;

use config::MapDefinition;
use loaded::Margins;

/// Loaded representation of a `Map`.
#[derive(Clone, Debug, PartialEq, new)]
pub struct Map {
    /// Map configuration.
    pub definition: MapDefinition,
    /// Coordinates of the limits of the playable area.
    pub margins: Margins,
    /// Default material for layer entities.
    pub sprite_material_mesh: Option<SpriteMaterialMesh>,
    /// Handle to the animations for this map's layers.
    pub animations: Option<Vec<Handle<Animation<Material>>>>,
}

impl Asset for Map {
    const NAME: &'static str = "map_model::loaded::Map";
    type Data = Self;
    type HandleStorage = VecStorage<Handle<Self>>;
}

impl Component for Map {
    type Storage = DenseVecStorage<Self>;
}

impl From<Map> for Result<ProcessingState<Map>, Error> {
    fn from(map: Map) -> Result<ProcessingState<Map>, Error> {
        Ok(ProcessingState::Loaded(map))
    }
}

/// Handle to a Map
pub type MapHandle = Handle<Map>;
