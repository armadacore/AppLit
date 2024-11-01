use crate::core::applit::lib::cache::read_binary_file;
use crate::core::applit::lib::directory::app_location_path;
use crate::core::applit::lib::node::try_create_node_from_source;
use crate::core::applit::lib::target::app_target_mode;
use crate::core::feedback::error::Cause;
use crate::core::parser::node::AstNode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

mod ast;
mod entry;
mod location;
mod mode;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AppLitMode {
    SourceCode,
    ByteCode,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppLitAst {
    pub references: HashMap<String, usize>,
    pub nodes: Vec<AstNode>,
}

pub struct AppLit {
    location: PathBuf,
    entry: PathBuf,
    mode: AppLitMode,
    ast: Option<Arc<Mutex<AppLitAst>>>,
}

impl AppLit {
    pub fn new(app_directory: &str) -> Result<Self, Cause> {
        let location = app_location_path(app_directory)?;
        let (mode, entry) = app_target_mode(&location)?;

        Ok(Self {
            location,
            entry,
            mode,
            ast: Some(Arc::new(Mutex::new(AppLitAst {
                references: HashMap::new(),
                nodes: Vec::new(),
            }))),
        })
    }

    pub fn run(&mut self) -> Result<AppLitAst, Cause> {
        let result = match try_create_node_from_source(self)? {
            false => read_binary_file(self)?,
            true => {
                match Arc::try_unwrap(self.ast.take().unwrap()) {
                    Ok(ast) => {
                        // write_binary_file(self)?;
                        ast.into_inner().unwrap()
                    }
                    Err(e) => return Err(Cause::MutexUnwrapError("For AppLit.ast".into())),
                }
            }
        };

        Ok(result)
    }
}
