use crate::core::feedback::ErrorCause;
use crate::core::parser::statements::chained_property::{
    parse_chained_property, ChainedProperties,
};
use crate::core::tokenizer::{snapshot_error, TokenDeclaration, TokenSnapshot, Tokens};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectDeclaration {
    pub objects: Vec<(TokenSnapshot, ChainedProperties)>,
}

pub fn parse_object_declaration(
    tokens: &mut Tokens,
) -> Result<ObjectDeclaration, ErrorCause> {
    let mut objects: Vec<(TokenSnapshot, ChainedProperties)> = Vec::new();

    while tokens.peek().is_some() {
        if let Some(TokenDeclaration::BlockOpen(_)) = tokens.peek() {
            tokens.next();
            continue;
        }
        if let Some(TokenDeclaration::StatementDivider(_)) = tokens.peek() {
            tokens.next();
            continue;
        }

        
        if is_declaration(tokens) {
            let identifier = tokens.next().unwrap().extract_snapshot();

            if let Some(TokenDeclaration::StatementAssignment(_)) = tokens.peek() {
                tokens.next();
                let chained_properties = parse_chained_property(tokens)?;
                objects.push((identifier, chained_properties));

                continue;
            }

            objects.push((identifier.clone(), ChainedProperties::new(vec![identifier.clone()])));
            continue;
        }

        if let Some(TokenDeclaration::BlockClose(_)) = tokens.peek() {
            tokens.next();
            return Ok(ObjectDeclaration{ objects });
        }

        return Err(snapshot_error(tokens.peek()));
    }

    Err(snapshot_error(tokens.peek()))
}

fn is_declaration(tokens: &mut Tokens) -> bool {
    if matches!(tokens.peek(), Some(TokenDeclaration::Literal(_)) | Some(TokenDeclaration::Identifier(_))) {
        return true;
    }
    
    false
}
