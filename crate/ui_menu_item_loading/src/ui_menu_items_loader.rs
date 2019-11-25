use asset_model::loaded::{AssetId, AssetIdMappings};
use game_mode_selection_model::GameModeIndex;
use sequence_model::loaded::AssetSequenceIdMappings;
use sprite_model::config::SpriteSequenceName;
use ui_menu_item_model::{
    config,
    loaded::{UiMenuItem, UiMenuItems},
};

/// Loads `UiMenuItem`s from items.
#[derive(Debug)]
pub struct UiMenuItemsLoader<'s> {
    /// `AssetIdMappings`.
    pub asset_id_mappings: &'s AssetIdMappings,
    /// `AssetSequenceIdMappings`.
    pub asset_sequence_id_mappings_sprite: &'s AssetSequenceIdMappings<SpriteSequenceName>,
}

impl<'s> UiMenuItemsLoader<'s> {
    /// Loads `UiMenuItems`.
    ///
    /// # Parameters
    ///
    /// * `item_iterator`: Iterator over the items from which to extract the asset data.
    /// * `asset_id`: Asset ID to store the asset data against.
    pub fn items_to_datas<'f, ItemIterator>(
        &self,
        item_iterator: ItemIterator,
        asset_id: AssetId,
    ) -> UiMenuItems<GameModeIndex>
    where
        ItemIterator: Iterator<Item = &'f config::UiMenuItem<GameModeIndex>>,
    {
        let ui_menu_items = item_iterator
            .map(|ui_menu_item| {
                let sequence_id_mappings = self
                    .asset_sequence_id_mappings_sprite
                    .get(asset_id)
                    .unwrap_or_else(|| {
                        let asset_slug = self
                            .asset_id_mappings
                            .slug(asset_id)
                            .expect("Expected `AssetSlug` to exist.");
                        panic!(
                            "Expected `SequenceIdMappings<SpriteSequenceName>` to exist for `{}`.",
                            asset_slug
                        )
                    });
                let sequence = &ui_menu_item.sprite.sequence;
                let sequence_id = sequence_id_mappings
                    .id(sequence)
                    .copied()
                    .unwrap_or_else(|| {
                        let asset_slug = self
                            .asset_id_mappings
                            .slug(asset_id)
                            .expect("Expected `AssetSlug` to exist.");
                        panic!(
                            "Expected `SequenceIdMapping` to exist for sequence `{}` for asset \
                             `{}`.",
                            sequence, asset_slug
                        )
                    });

                UiMenuItem::new(sequence_id, ui_menu_item.index)
            })
            .collect::<Vec<UiMenuItem<GameModeIndex>>>();

        UiMenuItems::new(ui_menu_items)
    }
}
