use crate::bin::constants;
use crate::composer::{AppLit, AppLitAst};
use crate::core::feedback::ErrorCause;
use bincode;
use sha3::{Digest, Sha3_256};
use std::fs::File;
use std::io::{Read, Write};

pub fn write_binary_file(app_lit: &AppLit) -> Result<(), ErrorCause> {
    let path_buf = app_lit.location.join(constants::BINARY_CODE_FILE);
    match path_buf.to_str() {
        Some(path) => {
            let encoded = bincode::serialize(&app_lit.ast);
            if encoded.is_err() {
                return Err(ErrorCause::CouldNotSerializeData("AstNode".into()));
            }
            let encoded = encoded.unwrap();
            let mut hasher = Sha3_256::new();
            hasher.update(&encoded);
            let hash = hasher.finalize();

            let file = File::create(path);
            if file.is_err() {
                return Err(ErrorCause::CouldNotCreateFile(path.into()));
            }
            let mut file = file.unwrap();

            let write = file.write_all(&encoded);
            if write.is_err() {
                return Err(ErrorCause::CouldNotWriteFile(path.into()));
            }

            let write = file.write_all(&hash);
            if write.is_err() {
                return Err(ErrorCause::CouldNotWriteFile(path.into()));
            }
        },
        None => return Err(ErrorCause::UnexpectedError("Could not convert path to string".into())),
    };

    Ok(())
}

pub fn read_binary_file(app_lit: &AppLit) -> Result<Option<AppLitAst>, ErrorCause> {
    let path = app_lit.entry.to_str().unwrap().to_string();

    match File::open(&app_lit.entry) {
        Ok(mut file) => {
            let mut data = Vec::<u8>::new();

            if file.read_to_end(&mut data).is_err() {
                return Err(ErrorCause::CouldNotReadFile(path));
            }
            let (encoded_data, stored_hash) = data.split_at(data.len() - 32);
            let mut hasher = Sha3_256::new();
            hasher.update(encoded_data);
            let computed_hash = hasher.finalize();

            if stored_hash != computed_hash.as_slice() {
                return Err(ErrorCause::UnexpectedError("File is modification".into()));
            }

            let result = bincode::deserialize::<Option<AppLitAst>>(encoded_data);

            if result.is_err() {
                return Err(ErrorCause::CouldNotDeserializeData(path));
            }

            Ok(result.unwrap())
        }
        Err(_) => Err(ErrorCause::CouldNotOpenFile(path))
    }
}