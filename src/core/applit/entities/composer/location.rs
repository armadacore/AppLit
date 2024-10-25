use crate::bin::constants;
use crate::composer::AppLit;
use std::path::{Path, PathBuf};

impl AppLit{
    pub fn get_joined_location(&self, path: &str) -> PathBuf {
        let uri =if path.starts_with(constants::URI_DIVIDER) {
            path.strip_prefix(constants::URI_DIVIDER).expect("Location path could not be stripped from stripped.")
        } else {
            path
        };

        self.location.join(uri.trim_matches('\''))
    }

    pub fn get_module_path<P: AsRef<Path>>(&self, location: P) -> PathBuf {
        let mut path = location.as_ref().to_path_buf();
        path.set_extension(constants::MODULE_EXTENSION);
        path
    }
}