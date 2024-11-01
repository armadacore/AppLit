use crate::bin::constants;
use crate::core::applit::entities::bundle::{AppLit, AppLitAst};
use crate::core::feedback::error::Cause;
use bincode;
use sha3::{Digest, Sha3_256};
use std::fs::File;
use std::io::{Read, Write};

pub fn write_binary_file(app_lit: &AppLit) -> Result<(), Cause> {
    let path_buf = app_lit.get_joined_location(constants::BINARY_CODE_FILE);
    match path_buf.to_str() {
        Some(path) => {
            let ast = app_lit.get_ast()?;
            let encoded = bincode::serialize(&*ast);

            if encoded.is_err() {
                return Err(Cause::CouldNotSerializeData("AstNode".into()));
            }
            let encoded = encoded.unwrap();
            let mut hasher = Sha3_256::new();
            hasher.update(&encoded);
            let hash = hasher.finalize();

            let file = File::create(path);
            if file.is_err() {
                return Err(Cause::CouldNotCreateFile(path.into()));
            }
            let mut file = file.unwrap();

            let write = file.write_all(&encoded);
            if write.is_err() {
                return Err(Cause::CouldNotWriteFile(path.into()));
            }

            let write = file.write_all(&hash);
            if write.is_err() {
                return Err(Cause::CouldNotWriteFile(path.into()));
            }
        }
        None => return Err(Cause::UnexpectedError("Could not convert path to string".into())),
    };

    Ok(())
}

pub fn read_binary_file(app_lit: &AppLit) -> Result<AppLitAst, Cause> {
    let entry_path = app_lit.get_entry();

    match File::open(&entry_path) {
        Ok(mut file) => {
            let mut data = Vec::<u8>::new();

            if file.read_to_end(&mut data).is_err() {
                return Err(Cause::CouldNotReadFile(entry_path));
            }
            let (encoded_data, stored_hash) = data.split_at(data.len() - 32);
            let mut hasher = Sha3_256::new();
            hasher.update(encoded_data);
            let computed_hash = hasher.finalize();

            if stored_hash != computed_hash.as_slice() {
                return Err(Cause::UnexpectedError("File is modification".into()));
            }

            let result = bincode::deserialize::<AppLitAst>(encoded_data);

            if result.is_err() {
                return Err(Cause::CouldNotDeserializeData(entry_path));
            }

            Ok(result.unwrap())
        }
        Err(_) => Err(Cause::CouldNotOpenFile(entry_path)),
    }
}
