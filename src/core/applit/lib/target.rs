use crate::bin::constants;
use crate::core::applit::entities::bundle::AppLitMode;
use crate::core::feedback::error::Cause;
use std::path::{Path, PathBuf};

pub fn app_target_mode(app_location: &Path) -> Result<(AppLitMode, PathBuf), Cause>{
    let mut app_entry_path = app_location.join(constants::BINARY_CODE_FILE);
    let mut mode = AppLitMode::ByteCode;

    if !app_entry_path.exists() && !app_entry_path.is_file() {
        mode = AppLitMode::SourceCode;
        app_entry_path = app_location.join(constants::SOURCE_CODE_FILE);

        if !app_entry_path.exists() || !app_entry_path.is_file() {
            let path = app_location.to_str().unwrap();
            return Err(Cause::EntryNotFound(path.into()));
        }
    }

    Ok((mode, app_entry_path))
}