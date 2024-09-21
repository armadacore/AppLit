use std::path::{Path, PathBuf};
use crate::feedback::error::{file_not_found, path_not_found, ErrorFeedback};

pub enum AstMode {
    Binary,
    Raw
}

pub struct AstOperation {
    pub mode: AstMode,
    pub main_path: PathBuf,
}

pub fn create(root_dir: &str) -> Result<String, ErrorFeedback>{
    let root_path_exists = exists_root_dir(root_dir)?;
    let main_file_exists = exists_main_file(&root_path_exists)?;
    
    Ok(String::from(root_dir))
}

fn exists_root_dir(root_dir: &str) -> Result<PathBuf, ErrorFeedback>{
    let path = Path::new(root_dir).to_owned();

    if path.is_dir() {
        Ok(path)
    } else {
        Err(path_not_found(root_dir))
    }
}

fn exists_main_file(root_dir: &Path) -> Result<AstOperation, ErrorFeedback>{
    let main_app_file = root_dir.join("main.app");
    
    if main_app_file.is_file() {
       Ok(AstOperation{
            mode: AstMode::Raw,
            main_path: main_app_file,
        })
    } else {
        let main_applit_file = root_dir.join("main.applit");

        if main_applit_file.is_file() {
            Ok(AstOperation {
                mode: AstMode::Binary,
                main_path: main_app_file,
            })
        } else {
            let root_path = root_dir.to_str().unwrap();
            let err_msg = format!("main.app or main.applit in {root_path}");
            
            Err(file_not_found(&err_msg))
        }
    }
}