use crate::bin::constants;
use crate::core::applit::lib::directory::app_location_path;
use crate::core::applit::lib::target::app_target_mode;
use crate::core::feedback::ErrorCause;
use crate::core::parser::{module_tree_builder, AstNode};
use crate::core::tokenizer::tokenize_file;
use crate::mode::AppLitMode;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::vec;

pub struct AppLit {
    pub location: PathBuf,
    entry: PathBuf,
    mode: AppLitMode,
    nodes: Vec<AstNode>,
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
        if self.mode == AppLitMode::SourceCode {
            let tokens = tokenize_file(&self.entry);
            let nodes = module_tree_builder(tokens)?;
            
            self.nodes.push(nodes);
            
            return Ok(self.nodes.clone());
        }

        self.read_binary_file()
    }

    pub fn cache_and_run(&mut self) -> Result<Vec<AstNode>, ErrorCause>{
        if self.mode == AppLitMode::SourceCode {
            let tokens = tokenize_file(&self.entry);
            let nodes = module_tree_builder(tokens)?;
            
            self.nodes.push(nodes);
            
            self.write_binary_file()?;

            return Ok(self.nodes.clone());
        }

        self.read_binary_file()
    }

    fn write_binary_file(&self) -> Result<(), ErrorCause> {
        let path_buf = self.location.join(constants::BINARY_CODE_FILE);
        match path_buf.to_str() {
            Some(path) => {
                let encoded = bincode::serialize(&self.nodes);
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

    fn read_binary_file(&self) -> Result<Vec<AstNode>, ErrorCause> {
        let path = self.entry.to_str().unwrap().to_string();

        match File::open(&self.entry) {
            Ok(mut file) => {
                let mut encoded = Vec::<u8>::new();

                if file.read_to_end(&mut encoded).is_err() {
                    return Err(ErrorCause::CouldNotReadFile(path));
                }

                let result = bincode::deserialize::<Vec<AstNode>>(&encoded);

                if result.is_err() {
                    return Err(ErrorCause::CouldNotDeserializeData(path));
                }

                Ok(result.unwrap())
            }
            Err(_) => Err(ErrorCause::CouldNotOpenFile(path))
        }
    }
}