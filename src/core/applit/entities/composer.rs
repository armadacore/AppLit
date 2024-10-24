use crate::core::applit::lib::cache::{read_binary_file, write_binary_file};
use crate::core::applit::lib::directory::app_location_path;
use crate::core::applit::lib::node::try_create_node_from_source;
use crate::core::applit::lib::target::app_target_mode;
use crate::core::feedback::ErrorCause;
use crate::core::parser::AstNode;
use crate::mode::AppLitMode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppLitAst {
    references: HashMap<PathBuf, usize>,
    nodes: Vec<AstNode>,
}

pub struct AppLit {
    pub location: PathBuf,
    pub entry: PathBuf,
    pub mode: AppLitMode,
    pub ast: Option<AppLitAst>,
}

impl AppLit {
    pub fn new(app_directory: &str) -> Result<Self, ErrorCause> {
        let location = app_location_path(app_directory)?;
        let (mode, entry) = app_target_mode(&location)?;

        Ok(Self {
            location,
            entry,
            mode,
            ast: Some(AppLitAst {
                references: HashMap::new(),
                nodes: Vec::new(),
            }),
        })
    }

    pub fn run(&mut self) -> Result<AppLitAst, ErrorCause> {
        let result = match try_create_node_from_source(self)? {
            None => read_binary_file(self)?.take().unwrap(),
            Some(ast_node) => {
                if let Some(ast) = &mut self.ast {
                    ast.nodes.push(ast_node);
                    write_binary_file(self)?;
                }

                self.ast.take().unwrap()
            }
        };

        Ok(result)
    }
}
