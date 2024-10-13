#![allow(unused_variables)]
#![allow(dead_code)]

use crate::bin::constants;
use crate::core::feedback::ErrorCause;
use crate::core::parser::{main_tree_builder, module_tree_builder, AstNode, AstNodeMain};
use crate::core::tokenizer::tokenize_file;
use std::path::PathBuf;

mod bin;

mod core;

#[derive(PartialEq)]
pub enum AppLitMode {
    SourceCode,
    ByteCode,
}

pub struct AppLit {
    pub app_directory: PathBuf,
    app_entry_path: PathBuf,
    mode: AppLitMode,
    node: AstNodeMain,
}

impl AppLit {
    pub fn new(app_directory_path: &str) -> Result<Self, ErrorCause> {
        let app_directory = PathBuf::from(app_directory_path);
        let mut app_entry_path = app_directory.join(constants::BINARY_CODE_FILE);
        let mut mode = AppLitMode::ByteCode;

        if !app_directory.exists() {
            return Err(ErrorCause::PathNotFound(app_directory_path));
        }
        if !app_directory.is_dir() {
            return Err(ErrorCause::DirectoryNotFound(app_directory_path));
        }
        if !app_entry_path.exists() && !app_entry_path.is_file() {
            mode = AppLitMode::SourceCode;
            app_entry_path = app_directory.join(constants::SOURCE_CODE_FILE);

            if !app_entry_path.exists() || !app_entry_path.is_file() {
                return Err(ErrorCause::EntryNotFound(app_directory_path));
            }
        }

        Ok(Self {
            app_directory,
            app_entry_path,
            mode,
            node: AstNodeMain::Statements(vec![]),
        })
    }

    pub fn run(&self) -> Result<AstNode, ErrorCause> {
        let tokens = tokenize_file(&self.app_entry_path);

        if self.mode == AppLitMode::SourceCode {
            return module_tree_builder(tokens);
        }

        main_tree_builder(tokens)
    }
}
