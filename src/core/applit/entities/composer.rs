use crate::core::applit::lib::cache::{read_binary_file, write_binary_file};
use crate::core::applit::lib::directory::app_location_path;
use crate::core::applit::lib::node::try_create_node_from_source_code;
use crate::core::applit::lib::target::app_target_mode;
use crate::core::feedback::ErrorCause;
use crate::core::parser::AstNode;
use crate::mode::AppLitMode;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::vec;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppLit {
    pub location: PathBuf,
    pub entry: PathBuf,
    pub mode: AppLitMode,
    pub nodes: Vec<AstNode>,
}

impl AppLit {
    pub fn new(app_directory: &str) -> Result<Self, ErrorCause> {
        let location = app_location_path(app_directory)?;
        let (mode, entry ) = app_target_mode(&location)?;

        Ok(Self {
            location,
            entry,
            mode,
            nodes: vec![],
        })
    }

    pub fn run(&mut self) -> Result<Self, ErrorCause> {
        let mut result = self.clone();
        
        match try_create_node_from_source_code(self)? {
            None => {
                result = read_binary_file(self)?;
                result.entry = self.entry.clone();
                result.mode = self.mode.clone();
            },
            Some(ast_node) => {
                result.nodes.push(ast_node);
                write_binary_file(&result)?;
            }
        };

        Ok(result)
    }
}