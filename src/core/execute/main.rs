use crate::bin::constants;
use crate::feedback::error::ErrorCause;
use crate::token;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum MainOperationMode {
    AppLit,
    App,
}

#[derive(Debug)]
pub struct ExecuteMainOperation {
    pub mode: MainOperationMode,
    pub file_path: PathBuf,
}

pub fn new(root_dir: &str) -> Result<(), ErrorCause> {
    let root_path_exists = exists_dir(root_dir)?;
    let exo = get_main_file_execute_operation(&root_path_exists)?;
    let result = match exo.mode {
        MainOperationMode::App => token::main_declaration(&exo.file_path),
        MainOperationMode::AppLit => todo!("read binary file and return [Ast]"),
    }?;

    for res in result {
        println!("Result: {res:?}");
    }

    Ok(())
}

fn exists_dir(root_dir: &str) -> Result<PathBuf, ErrorCause> {
    let path = Path::new(root_dir).to_owned();

    if path.is_dir() {
        Ok(path)
    } else {
        Err(ErrorCause::PathNotFound(root_dir.into()))
    }
}

fn get_main_file_execute_operation(root_dir: &Path) -> Result<ExecuteMainOperation, ErrorCause> {
    let main_app_file = root_dir.join(constants::MAIN_APP_FILE);
    if main_app_file.is_file() {
        return Ok(ExecuteMainOperation {
            mode: MainOperationMode::App,
            file_path: main_app_file,
        });
    }

    let main_applit_file = root_dir.join(constants::MAIN_APPLIT_FILE);
    if main_applit_file.is_file() {
        return Ok(ExecuteMainOperation {
            mode: MainOperationMode::AppLit,
            file_path: main_app_file,
        });
    }

    let root_path = root_dir.to_str().unwrap();
    let main_app = constants::MAIN_APP_FILE;
    let main_applit = constants::MAIN_APPLIT_FILE;
    let err_msg = format!("{main_app} or {main_applit} in {root_path}");

    Err(ErrorCause::FileNotFound(err_msg))
}
