use crate::bin::constants;
use crate::composer::AppLit;
use crate::core::feedback::ErrorCause;
use crate::core::parser::AstNode;
use std::fs::File;
use std::io::{Read, Write};

pub fn write_binary_file(app_lit: &AppLit) -> Result<(), ErrorCause> {
    let path_buf = app_lit.location.join(constants::BINARY_CODE_FILE);
    match path_buf.to_str() {
        Some(path) => {
            let encoded = bincode::serialize(&app_lit.nodes);
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

pub fn read_binary_file(app_lit: &AppLit) -> Result<Vec<AstNode>, ErrorCause> {
    let path = app_lit.entry.to_str().unwrap().to_string();

    match File::open(&app_lit.entry) {
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