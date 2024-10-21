use crate::core::applit::lib::cache::{read_binary_file};
use crate::core::applit::lib::directory::app_location_path;
use crate::core::applit::lib::node::create_node_from_source_code;
use crate::core::applit::lib::target::app_target_mode;
use crate::core::feedback::ErrorCause;
use crate::core::parser::AstNode;
use crate::mode::AppLitMode;
use std::path::PathBuf;
use std::vec;

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

    pub fn run(&mut self) -> Result<Vec<AstNode>, ErrorCause> {
        let node = create_node_from_source_code(self)?;
        
        match node {
            None => read_binary_file(self),
            Some(ast_node) => {
                self.nodes.push(ast_node);
                // write_binary_file(self)?;
                
                Ok(self.nodes.clone())
            }
        }
    }
}