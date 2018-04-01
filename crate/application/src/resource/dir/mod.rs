//! Constants for resource directories.

mod discovery_context;

pub use self::discovery_context::DiscoveryContext;

use std::env;
use std::io;
use std::path::PathBuf;

use resource::Error;

// Note to self:
//
// I know in code we use the singular form of the noun, whereas the directory names are plural.
// This is in line with Amethyst's convention of resource directories.

/// `assets` directory name.
pub const ASSETS: &str = "assets";
/// `resources` directory name.
pub const RESOURCES: &str = "resources";

/// Returns an absolute path to the current exe's assets directory.
///
/// # Errors
///
/// Returns a [`resource::Error`][res_err] with error kind [`ErrorKind::DirDiscovery`][dir_disc]
/// when the following scenarios occur:
///
/// * Unable to retrieve current executable path.
/// * Unable to retrieve current executable parent.
///
/// [res_err]: resource/struct.Error.html
/// [dir_disc]: resource/enum.ErrorKind.html#variant.DirDiscovery
pub fn assets_dir() -> Result<PathBuf, Error> {
    assets_dir_internal(env::current_exe())
}

#[inline]
fn assets_dir_internal(current_exe_result: io::Result<PathBuf>) -> Result<PathBuf, Error> {
    let current_exe = current_exe_result.map_err(|io_error| -> Error {
        DiscoveryContext::new(
            None,
            ASSETS,
            "Failed to get current executable path.",
            Some(io_error),
        ).into()
    })?;

    {
        // Makes no sense if the current executable has no parent directory.
        let exe_dir = current_exe
            .parent()
            .expect("Expected current exe to have a parent directory");

        // Check if directory exists.
        let dir = exe_dir.join(ASSETS);
        if dir.is_dir() {
            return Ok(dir);
        }
    }

    return Err(DiscoveryContext::new(
        Some(current_exe),
        ASSETS,
        "Failed to assert that directory exists and can be accessed.",
        None,
    ).into());
}

#[cfg(test)]
mod test {
    use std::fs;
    use std::io;
    use std::path::PathBuf;

    use tempfile::tempdir;

    use super::{assets_dir_internal, ASSETS};
    use resource::dir;
    use resource::{Error, ErrorKind};

    fn asset_dir_discovery_error(
        expected_context: dir::DiscoveryContext,
        assets_dir: Result<PathBuf, Error>,
    ) {
        match *assets_dir.unwrap_err().kind() {
            ErrorKind::DirDiscovery(ref discovery_context) => {
                assert_eq!(expected_context, *discovery_context);
            }
            // kcov-ignore-start
            ref kind @ _ => {
                panic!(
                    "Expected error with kind `{:?}` but was `{:?}`",
                    ErrorKind::DirDiscovery(expected_context),
                    kind
                );
            } // kcov-ignore-end
        }
    }

    #[test]
    fn assets_dir_returns_assets_dir_path_beside_current_executable() {
        let exe_dir = tempdir().unwrap();
        let exe_path = exe_dir.path().join("current_exe");
        let _assets_dir = fs::create_dir(exe_dir.path().join(ASSETS)).unwrap();
        let assets_dir = assets_dir_internal(Ok(exe_path));

        // `error-chain` generated `Error` doesn't implement `PartialEq`, so we have to manually
        // compare
        let expected: Result<PathBuf, Error> = Ok(exe_dir.path().join(ASSETS));
        assert!(
            assets_dir.is_ok(),
            "Expected assets_dir to return {:?}, but was {:?}",
            expected,
            assets_dir
        );
        assert_eq!(expected.unwrap(), assets_dir.unwrap());
    }

    #[test]
    fn assets_dir_returns_contextual_error_when_failing_to_get_current_exe_path() {
        let assets_dir = assets_dir_internal(Err(io::Error::new(io::ErrorKind::Other, "oh no!")));

        let expected_discovery_context = dir::DiscoveryContext::new(
            None,
            ASSETS,
            "Failed to get current executable path.",
            Some(io::Error::new(io::ErrorKind::Other, "oh no!")),
        ); // kcov-ignore

        asset_dir_discovery_error(expected_discovery_context, assets_dir);
    }

    #[test]
    fn assets_dir_returns_contextual_error_when_assets_dir_does_not_exist() {
        let exe_dir = tempdir().unwrap();
        let exe_path = exe_dir.path().join("current_exe");
        let assets_dir = assets_dir_internal(Ok(exe_path.clone()));

        let expected_discovery_context = dir::DiscoveryContext::new(
            Some(exe_path),
            ASSETS,
            "Failed to assert that directory exists and can be accessed.",
            None,
        );
        asset_dir_discovery_error(expected_discovery_context, assets_dir);
    }
}
