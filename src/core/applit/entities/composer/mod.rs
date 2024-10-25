use crate::core::applit::lib::cache::read_binary_file;
use crate::core::applit::lib::directory::app_location_path;
use crate::core::applit::lib::node::try_create_node_from_source;
use crate::core::applit::lib::target::app_target_mode;
use crate::core::feedback::ErrorCause;
use crate::core::parser::AstNode;
use crate::mode::AppLitMode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

mod ast;
mod entry;
mod mode;
mod location;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppLitAst {
    pub references: HashMap<PathBuf, usize>,
    pub nodes: Vec<AstNode>,
}

pub struct AppLit {
    location: PathBuf,
    entry: PathBuf,
    mode: AppLitMode,
    ast: Option<Arc<Mutex<AppLitAst>>>,
}

impl AppLit {
    pub fn new(app_directory: &str) -> Result<Self, ErrorCause> {
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

    pub fn run(&mut self) -> Result<AppLitAst, ErrorCause> {
        let result = match try_create_node_from_source(self)? {
            false => read_binary_file(self)?,
            true => {
                // write_binary_file(self)?;
                Arc::try_unwrap(self.ast.take().unwrap())
                    .map_err(|_| ErrorCause::MutexUnwrapError("For AppLit.ast".into()))?
                    .into_inner()
                    .unwrap()
            }
        };

        Ok(result)
    }
}
