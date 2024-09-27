use crate::bin::constants;
use crate::core::execute::token;
use crate::feedback::error::{file_not_found, path_not_found, ErrorFeedback};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum MainOperationMode {
    AppLit,
    App
}

#[derive(Debug)]
pub struct ExecuteMainOperation {
    pub mode: MainOperationMode,
    pub file_path: PathBuf,
}

pub fn new(root_dir: &str) -> Result<(), ErrorFeedback>{
    let root_path_exists = exists_dir(root_dir)?;
    let exo = get_main_file_execute_operation(&root_path_exists)?;
    let result = match exo.mode {
        MainOperationMode::App => token::main::declaration(&exo.file_path),
        MainOperationMode::AppLit => todo!("read binary file and return [Ast]"),
    }?;

    // for res in result{
    //     println!("Result: {res:?}");
    // }

    Ok(())
}

fn exists_dir(root_dir: &str) -> Result<PathBuf, ErrorFeedback>{
    let path = Path::new(root_dir).to_owned();

    if path.is_dir() {
        Ok(path)
    } else {
        Err(path_not_found(root_dir))
    }
}

fn get_main_file_execute_operation(root_dir: &Path) -> Result<ExecuteMainOperation, ErrorFeedback>{
    let main_app_file = root_dir.join(constants::MAIN_APP_FILE);
    if main_app_file.is_file() {
        return Ok(ExecuteMainOperation {
            mode: MainOperationMode::App,
            file_path: main_app_file,
        })
    }

    let main_applit_file = root_dir.join(constants::MAIN_APPLIT_FILE);
    if main_applit_file.is_file() {
        return Ok(ExecuteMainOperation {
            mode: MainOperationMode::AppLit,
            file_path: main_app_file,
        })
    }

    let root_path = root_dir.to_str().unwrap();
    let main_app = constants::MAIN_APP_FILE;
    let main_applit = constants::MAIN_APPLIT_FILE;
    let err_msg = format!("{main_app} or {main_applit} in {root_path}");

    Err(file_not_found(&err_msg))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bin::mock_constants;
    use crate::feedback::error;

    #[test]
    fn exists_of_root_dir_is_true() {
        let expected_path = PathBuf::from(mock_constants::ROOT_DIR);

        match exists_dir(mock_constants::ROOT_DIR) {
            Ok(actual_path) => assert_eq!(actual_path, expected_path),
            Err(_) => panic!("Expected Ok, but got Err"),
        }
    }

    #[test]
    fn exists_of_root_dir_is_false(){
        let check_path = "/path/to/somewhere/else";
        let expected_error_message = format!("Path '{check_path}' not found");

        match exists_dir(check_path) {
            Ok(_) => error::panic("Expected Err, but got Ok"),
            Err(err) => assert_eq!(err.message, expected_error_message, "Error message seems to be wrong"),
        }
    }

    #[test]
    fn exists_of_main_file_is_true(){
        let root_path = PathBuf::from(mock_constants::ROOT_DIR);
        let expected_path = PathBuf::from(mock_constants::ROOT_DIR).join("main.app");

        match get_main_file_execute_operation(&root_path) {
            Ok(ast) => assert_eq!(ast.file_path, expected_path),
            Err(_) => panic!("Expected Ok, but got Err"),
        }
    }

    #[test]
    fn exists_of_main_file_is_false(){
        let check_path = "/path/to/somewhere/else";
        let root_path = PathBuf::from(check_path);
        let main_app = constants::MAIN_APP_FILE;
        let main_applit = constants::MAIN_APPLIT_FILE;
        let expected_error_message = format!("File '{main_app} or {main_applit} in {check_path}' not found");

        match get_main_file_execute_operation(&root_path) {
            Ok(_) => error::panic("Expected Err, but got Ok"),
            Err(err) => assert_eq!(err.message, expected_error_message, "Error message seems to be wrong"),
        }
    }
}