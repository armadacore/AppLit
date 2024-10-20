use crate::core::feedback::ErrorCause;
use std::path::PathBuf;

pub fn app_location_path(app_directory_path: &str) -> Result<PathBuf, ErrorCause> {
    let app_directory = PathBuf::from(app_directory_path);

    if !app_directory.exists() {
        return Err(ErrorCause::PathNotFound(app_directory_path.into()));
    }
    if !app_directory.is_dir() {
        return Err(ErrorCause::DirectoryNotFound(app_directory_path.into()));
    }

    Ok(app_directory)
}