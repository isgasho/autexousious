use std::path::PathBuf;

/// Contains the meta information about a configuration type.
///
/// This includes where the configuration is stored.
#[derive(Debug, Default, PartialEq, new)]
pub struct ConfigRecord {
    /// Directory path of the configuration relative to the assets directory.
    ///
    /// e.g. "default/objects/characters/heat"
    pub directory: PathBuf,
}
