use std::path::{Path, PathBuf};
use crate::bin::constants;
use crate::feedback::error::{file_not_found, path_not_found, ErrorFeedback};
use crate::tokenizer::tokens::{initialize, Token};

/// Define in which mode has the parser to run.
///
/// There are different between source-code (raw) and binary (already parsed).
#[derive(Debug)]
pub enum AstMode {
    /// The source-code is already parsed and stored as binary file
    AppLit,
    /// The source-code isn't parsed and has to tokenized before parsing
    App
}

/// Hold the information in which mode [`AstMode`] has to switch
/// and where the main file are located.
#[derive(Debug)]
pub struct AstOperation {
    /// Represents the mode [`AstMode`]
    pub mode: AstMode,
    /// Where is the main file located
    pub file_path: PathBuf,
}

/// Try to look up in the given root dir for main file.
///
/// It will look if the root dir still exists and as next
/// if [`constants::MAIN_APP`] or [`constants::MAIN_APPLIT`] still exists
///
/// # Parameters
/// * `root_dir: &str` - Root path where project is located and the main file can be found.
/// 
/// # Returns
/// * `Result<Ast, ErrorFeedback>` - `Ast` representation of tokenized source-code, [`ErrorFeedback`] the error which has occurred
pub fn look_up(root_dir: &str) -> Result<Vec<Token>, ErrorFeedback>{
    let root_path_exists = exists_dir(root_dir)?;
    let main_file_exists = exists_file(&root_path_exists)?;

    initialize(main_file_exists)
}

fn exists_dir(root_dir: &str) -> Result<PathBuf, ErrorFeedback>{
    let path = Path::new(root_dir).to_owned();

    if path.is_dir() {
        Ok(path)
    } else {
        Err(path_not_found(root_dir))
    }
}

fn exists_file(root_dir: &Path) -> Result<AstOperation, ErrorFeedback>{
    let main_app_file = root_dir.join(constants::MAIN_APP);
    if main_app_file.is_file() {
       return Ok(AstOperation{
            mode: AstMode::App,
            file_path: main_app_file,
        })
    }

    let main_applit_file = root_dir.join(constants::MAIN_APPLIT);
    if main_applit_file.is_file() {
        return Ok(AstOperation {
            mode: AstMode::AppLit,
            file_path: main_app_file,
        })
    }

    let root_path = root_dir.to_str().unwrap();
    let main_app = constants::MAIN_APP;
    let main_applit = constants::MAIN_APPLIT;
    let err_msg = format!("{main_app} or {main_applit} in {root_path}");

    Err(file_not_found(&err_msg))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bin::{mock_constants};
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

        match exists_file(&root_path) {
            Ok(ast) => assert_eq!(ast.file_path, expected_path),
            Err(_) => panic!("Expected Ok, but got Err"),
        }
    }

    #[test]
    fn exists_of_main_file_is_false(){
        let check_path = "/path/to/somewhere/else";
        let root_path = PathBuf::from(check_path);
        let main_app = constants::MAIN_APP;
        let main_applit = constants::MAIN_APPLIT;
        let expected_error_message = format!("File '{main_app} or {main_applit} in {check_path}' not found");

        match exists_file(&root_path) {
            Ok(_) => error::panic("Expected Err, but got Ok"),
            Err(err) => assert_eq!(err.message, expected_error_message, "Error message seems to be wrong"),
        }
    }
}