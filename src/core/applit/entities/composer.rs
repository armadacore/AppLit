use crate::bin::constants;
use crate::core::applit::lib::directory::app_location_path;
use crate::core::applit::lib::target::app_target_mode;
use crate::core::feedback::ErrorCause;
use crate::core::parser::{module_tree_builder, AstMainNode, AstNode};
use crate::core::tokenizer::tokenize_file;
use crate::mode::AppLitMode;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

pub struct AppLit {
    pub location: PathBuf,
    entry: PathBuf,
    mode: AppLitMode,
    node: AstMainNode,
}

impl AppLit {
    pub fn new(app_directory: &str) -> Result<Self, ErrorCause> {
        let location = app_location_path(app_directory)?;
        let (mode, entry ) = app_target_mode(&location)?;

        Ok(Self {
            location,
            entry,
            mode,
            node: AstMainNode::Statements(vec![]),
        })
    }

    pub fn run(&self) -> Result<AstNode, ErrorCause> {
        if self.mode == AppLitMode::SourceCode {
            let tokens = tokenize_file(&self.entry);
            return module_tree_builder(tokens);
        }

        self.read_binary_file()
    }

    pub fn cache_and_run(&self) -> Result<AstNode, ErrorCause>{
        if self.mode == AppLitMode::SourceCode {
            let tokens = tokenize_file(&self.entry);
            let nodes = module_tree_builder(tokens)?;
            self.write_binary_file(&nodes)?;

            return Ok(nodes);
        }

        self.read_binary_file()
    }

    fn write_binary_file(&self, nodes: &AstNode) -> Result<(), ErrorCause> {
        let path_buf = self.location.join(constants::BINARY_CODE_FILE);
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
        let path = self.entry.to_str().unwrap().to_string();

        match File::open(&self.entry) {
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