use crate::bin::constants;
use crate::core::applit::lib::cache::read_binary_file;
use crate::core::applit::lib::directory::app_location_path;
use crate::core::applit::lib::node::try_create_node_from_source;
use crate::core::applit::lib::target::app_target_mode;
use crate::core::feedback::ErrorCause;
use crate::core::parser::AstNode;
use crate::mode::AppLitMode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard};

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

    pub fn get_ast(&self) -> Result<MutexGuard<AppLitAst>, ErrorCause> {
        if let Some(ast_mutex) = &self.ast {
            return Ok(ast_mutex.lock().unwrap());
        }

        Err(ErrorCause::UnexpectedError("Ast Mutex is None".into()))
    }

    pub fn get_entry(&self) -> String {
        self.entry.to_string_lossy().to_string()
    }

    pub fn get_mode(&self) -> AppLitMode {
        self.mode.clone()
    }

    pub fn get_joined_location(&self, path: &str) -> PathBuf {
        let uri =if path.starts_with(constants::URI_DIVIDER) {
            path.strip_prefix(constants::URI_DIVIDER).expect("Location path could not be stripped from stripped.")
        } else {
            path
        };

        self.location.join(uri.trim_matches('\''))
    }
    
    pub fn get_module_path<P: AsRef<Path>>(&self, location: P) -> PathBuf {
        let mut path = location.as_ref().to_path_buf();
        path.set_extension(constants::MODULE_EXTENSION);
        path
    }

    pub fn add_ast_node_item(&mut self, item_path: &str, item_value: AstNode) -> usize {
        if let Some(ast) = &mut self.ast {
            let mut ast = ast.lock().unwrap();

            ast.nodes.push(item_value);

            let index = ast.nodes.len() - 1;
            let path = self.location.join(item_path);

            ast.references.insert(path, index);

            return index;
        }

        panic!("Attempted to add a node item to a composer without valid AST.");
    }

    pub fn exist_ast_node_item(&self, item_path: &str) -> bool {
        if let Some(ast) = &self.ast {
            let ast = ast.lock().unwrap();
            let path = self.location.join(item_path);

            return ast.references.contains_key(&path);
        }

        panic!("Attempted to find a node without valid AST.");
    }
}
