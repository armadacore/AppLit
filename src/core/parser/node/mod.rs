use crate::core::applit::entities::bundle::AppLit;
use crate::core::parser::node::main::statement_parser::AstMainNode;
use crate::core::parser::node::module::statement_parser::AstModuleNode;
use serde::{Deserialize, Serialize};

mod main;

mod module;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AstNode {
    Main(AstMainNode),
    Module(AstModuleNode),
}

pub struct TreeBuilder<'a> {
    app_lit: &'a mut AppLit,
}

impl<'a> TreeBuilder<'a> {
    pub fn new(app_lit: &'a mut AppLit) -> Self {
        TreeBuilder { app_lit }
    }
}
