use std::path::Path;
use crate::token::declaration::DeclarationResult;
use crate::token::declaration::main::MainDeclaration;
use crate::token::declaration::module::{ModuleDeclaration};

mod declaration;

mod utils;

mod reader;

pub fn main_declaration(file_path: &Path) -> DeclarationResult<MainDeclaration>{
    declaration::main::token(file_path)
}

pub fn module_declaration(file_path: &Path) -> DeclarationResult<ModuleDeclaration>{
    declaration::module::token(file_path)
}