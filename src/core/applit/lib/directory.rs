use crate::core::feedback::error::Cause;
use std::path::PathBuf;

pub fn app_location_path(app_directory_path: &str) -> Result<PathBuf, Cause> {
    let app_directory = PathBuf::from(app_directory_path);

    if !app_directory.exists() {
        return Err(Cause::PathNotFound(app_directory_path.into()));
    }
    if !app_directory.is_dir() {
        return Err(Cause::DirectoryNotFound(app_directory_path.into()));
    }

    Ok(app_directory)
}