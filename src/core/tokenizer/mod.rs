use crate::core::tokenizer::declaration::{
    main::MainDeclaration, module::ModuleDeclaration, DeclarationResult,
};
use std::path::Path;

mod declaration;

mod utils;

mod reader;

pub fn main_declaration(file_path: &Path) -> DeclarationResult<MainDeclaration> {
    declaration::main::token(file_path)
}

pub fn module_declaration(file_path: &Path) -> DeclarationResult<ModuleDeclaration> {
    declaration::module::token(file_path)
}
