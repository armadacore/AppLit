#![allow(unused_variables)]
#![allow(dead_code)]

use crate::bin::constants;
use crate::core::feedback::ErrorCause;
use crate::core::parser::{module_tree_builder, AstMainNode, AstNode};
use crate::core::tokenizer::tokenize_file;
use bincode;
use std::fs::File;
use std::io::{Read, Write};
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
    node: AstMainNode,
}

impl AppLit {
    pub fn new(app_directory_path: &str) -> Result<Self, ErrorCause> {
        let app_directory = PathBuf::from(app_directory_path);
        let mut app_entry_path = app_directory.join(constants::BINARY_CODE_FILE);
        let mut mode = AppLitMode::ByteCode;

        if !app_directory.exists() {
            return Err(ErrorCause::PathNotFound(app_directory_path.into()));
        }
        if !app_directory.is_dir() {
            return Err(ErrorCause::DirectoryNotFound(app_directory_path.into()));
        }
        if !app_entry_path.exists() && !app_entry_path.is_file() {
            mode = AppLitMode::SourceCode;
            app_entry_path = app_directory.join(constants::SOURCE_CODE_FILE);

            if !app_entry_path.exists() || !app_entry_path.is_file() {
                return Err(ErrorCause::EntryNotFound(app_directory_path.into()));
            }
        }

        Ok(Self {
            app_directory,
            app_entry_path,
            mode,
            node: AstMainNode::Statements(vec![]),
        })
    }

    pub fn run(&self) -> Result<AstNode, ErrorCause> {
        if self.mode == AppLitMode::SourceCode {
            let tokens = tokenize_file(&self.app_entry_path);
            return module_tree_builder(tokens);
        }

        self.read_binary_file()
    }
    
    pub fn cache_and_run(&self) -> Result<AstNode, ErrorCause>{
        if self.mode == AppLitMode::SourceCode {
            let tokens = tokenize_file(&self.app_entry_path);
            let nodes = module_tree_builder(tokens)?;
            self.write_binary_file(&nodes)?;
            
            return Ok(nodes);
        }

        self.read_binary_file()
    }
    
    fn write_binary_file(&self, nodes: &AstNode) -> Result<(), ErrorCause> {
        let path_buf = self.app_directory.join(constants::BINARY_CODE_FILE);
        match path_buf.to_str() {
            Some(path) => {
                let encoded = bincode::serialize(nodes);
                if encoded.is_err() {
                    return Err(ErrorCause::CouldNotSerializeData("AstNode".into()));
                }
                let encoded = encoded.unwrap();

                let file = File::create(path);
                if file.is_err() {
                    return Err(ErrorCause::CouldNotCreateFile(path.into()));
                }
                let mut file = file.unwrap();

                let write = file.write_all(&encoded);
                if write.is_err() {
                    return Err(ErrorCause::CouldNotWriteFile(path.into()));
                }
            },
            None => return Err(ErrorCause::UnexpectedError("Could not convert path to string".into())),
        };
        
        Ok(())
    }
    
    fn read_binary_file(&self) -> Result<AstNode, ErrorCause> {
        let path = self.app_entry_path.to_str().unwrap().to_string();
        
        match File::open(&self.app_entry_path) {
            Ok(mut file) => {
                let mut encoded = Vec::<u8>::new();
                
                if file.read_to_end(&mut encoded).is_err() {
                    return Err(ErrorCause::CouldNotReadFile(path));
                }
                
                let result = bincode::deserialize::<AstNode>(&encoded);
                
                if result.is_err() {
                    return Err(ErrorCause::CouldNotDeserializeData(path));
                }
                
                Ok(result.unwrap())
            }
            Err(_) => Err(ErrorCause::CouldNotOpenFile(path))
        }
    }
}
