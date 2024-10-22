use crate::core::feedback::ErrorCause;
use crate::core::parser::statements::chained_property::ChainedProperties;
use crate::core::parser::TreeBuilder;
use crate::core::tokenizer::snapshot_error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectDeclaration {
    object: HashMap<String, ChainedProperties>
}

pub fn parse_object_declaration(builder: &mut TreeBuilder) -> Result<ObjectDeclaration, ErrorCause> {
    while builder.tokens.peek().is_some() {

    }

    Err(snapshot_error(builder.tokens.peek()))
}