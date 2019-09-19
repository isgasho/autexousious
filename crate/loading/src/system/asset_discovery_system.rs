use std::path::PathBuf;

use amethyst::{
    ecs::{System, World, Write},
    shred::{ResourceId, SystemData},
};
use asset_loading::AssetDiscovery;
use asset_model::{
    config::{AssetIndex, AssetType},
    loaded::{AssetId, AssetIdMappings, AssetTypeMappings},
};
use derivative::Derivative;
use derive_new::new;
use log::debug;
use slotmap::SecondaryMap;
use typename_derive::TypeName;

use crate::AssetLoadStatus;

/// Discovers assets and writes to `Option<AssetIndex>`.
#[derive(Debug, Default, TypeName, new)]
pub struct AssetDiscoverySystem {
    /// Path to the assets directory.
    assets_dir: PathBuf,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct AssetDiscoverySystemData<'s> {
    /// `Option<AssetIndex>` resource.
    #[derivative(Debug = "ignore")]
    asset_index: Write<'s, Option<AssetIndex>>,
    /// `AssetIdMappings` resource.
    #[derivative(Debug = "ignore")]
    asset_id_mappings: Write<'s, AssetIdMappings>,
    /// `AssetTypeMappings` resource.
    #[derivative(Debug = "ignore")]
    asset_type_mappings: Write<'s, AssetTypeMappings>,
    /// `SecondaryMap<AssetId, AssetLoadStatus>` resource.
    #[derivative(Debug = "ignore")]
    asset_id_to_status: Write<'s, SecondaryMap<AssetId, AssetLoadStatus>>,
    /// `SecondaryMap<AssetId, PathBuf>` resource.
    #[derivative(Debug = "ignore")]
    asset_id_to_path: Write<'s, SecondaryMap<AssetId, PathBuf>>,
}

impl<'s> System<'s> for AssetDiscoverySystem {
    type SystemData = AssetDiscoverySystemData<'s>;

    fn run(
        &mut self,
        AssetDiscoverySystemData {
            mut asset_index,
            mut asset_id_mappings,
            mut asset_type_mappings,
            mut asset_id_to_status,
            mut asset_id_to_path,
        }: Self::SystemData,
    ) {
        // TODO: Do a diff between existing index and directory based on a file watch / notify.
        // TODO: See <https://github.com/polachok/derive-diff>
        if asset_index.is_none() {
            let asset_index_discovered = AssetDiscovery::asset_index(&self.assets_dir);
            debug!("Indexed assets: {:?}", &asset_index_discovered);

            let capacity = asset_index_discovered
                .objects
                .values()
                .fold(0, |acc, records| acc + records.len())
                + asset_index_discovered.maps.len();
            asset_id_mappings.reserve(capacity);

            let asset_records_objects =
                asset_index_discovered
                    .objects
                    .iter()
                    .flat_map(|(object_type, asset_records)| {
                        let asset_type = AssetType::Object(*object_type);
                        asset_records
                            .iter()
                            .map(move |asset_record| (asset_type, asset_record))
                    });
            let asset_records_maps = asset_index_discovered
                .maps
                .iter()
                .map(|asset_record| (AssetType::Map, asset_record));
            let asset_records = asset_records_objects.chain(asset_records_maps);
            asset_records.for_each(|(asset_type, asset_record)| {
                let asset_id = asset_id_mappings.insert(asset_record.asset_slug.clone());

                debug!(
                    "Asset ID ({:?}): slug: `{}`, type: `{:?}`",
                    asset_id, &asset_record.asset_slug, asset_type
                );

                asset_type_mappings.insert(asset_id, asset_type);
                asset_id_to_status.insert(asset_id, AssetLoadStatus::New);
                asset_id_to_path.insert(asset_id, asset_record.path.clone());
            });

            *asset_index = Some(asset_index_discovered);
        }
    }
}
