use crate::core::feedback::ErrorCause;
use crate::core::parser::statements::chained_property::{
    parse_chained_property, ChainedProperties,
};
use crate::core::parser::TreeBuilder;
use crate::core::tokenizer::{snapshot_error, TokenDeclaration, TokenSnapshot};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectDeclaration {
    pub objects: Vec<(TokenSnapshot, ChainedProperties)>,
}

pub fn parse_object_declaration(
    builder: &mut TreeBuilder,
) -> Result<ObjectDeclaration, ErrorCause> {
    let mut objects: Vec<(TokenSnapshot, ChainedProperties)> = Vec::new();

    while builder.tokens.peek().is_some() {
        if let Some(TokenDeclaration::BlockOpen(_)) = builder.tokens.peek() {
            builder.tokens.next();
            continue;
        }
        if let Some(TokenDeclaration::StatementDivider(_)) = builder.tokens.peek() {
            builder.tokens.next();
            continue;
        }

        
        if is_declaration(builder) {
            let identifier = builder.tokens.next().unwrap().extract_snapshot();

            if let Some(TokenDeclaration::StatementAssignment(_)) = builder.tokens.peek() {
                builder.tokens.next();
                let chained_properties = parse_chained_property(builder)?;
                objects.push((identifier, chained_properties));

                continue;
            }

            objects.push((identifier.clone(), ChainedProperties::new(vec![identifier.clone()])));
            continue;
        }

        if let Some(TokenDeclaration::BlockClose(_)) = builder.tokens.peek() {
            builder.tokens.next();
            return Ok(ObjectDeclaration{ objects });
        }

        return Err(snapshot_error(builder.tokens.peek()));
    }

    Err(snapshot_error(builder.tokens.peek()))
}

fn is_declaration(builder: &mut TreeBuilder) -> bool {
    if matches!(builder.tokens.peek(), Some(TokenDeclaration::Literal(_)) | Some(TokenDeclaration::Identifier(_))) {
        return true;
    }
    
    false
}
