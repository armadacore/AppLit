use crate::core::tokenizer::TokenSnapshot;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum AstCommitment {
    Id(String, String),
    Icon(String),
    Name(String),
    Version(String),
    Description(String),
    Link(String),
    Domain{
        default: String,
        distribution: HashMap<String, String>,
    }
}

#[derive(Debug, PartialEq)]
pub enum AstNode {
    Program {
        commitment: Vec<AstCommitment>,
        statements: Vec<AstNode>,
    },
    Import {
        snapshot: TokenSnapshot,
        namespace: Option<TokenSnapshot>,
        identifiers: Vec<TokenSnapshot>,
        reference: TokenSnapshot,
    },
    Function {
        snapshot: TokenSnapshot,
        identifier: TokenSnapshot,
        arguments: Vec<String>,
        body: Vec<String>,
        result: Option<String>,
    }
}

#[derive(Debug, Clone)]
pub enum AstError {
    UnexpectedToken(TokenSnapshot),
    UnexpectedError(Option<TokenSnapshot>),
    UnexpectedEOF,
}

impl fmt::Display for AstError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AstError::UnexpectedToken(snapshot    ) => {
                write!(f, "Unexpected Error at {:#?} for {:#?}", snapshot.location, snapshot.token)
            }
            AstError::UnexpectedError(snapshot) => {
                if snapshot.is_some() {
                    let snapshot = snapshot.as_ref().unwrap();
                    return write!(f, "Unexpected Error at {:#?} for {:#?}", snapshot.location, snapshot.token)
                }

                write!(f, "Unexpected Error")
            }
            AstError::UnexpectedEOF => write!(f, "Unexpected EOF")
        }
    }
}