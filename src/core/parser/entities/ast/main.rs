use crate::core::parser::entities::ast::statements::import::ImportStatement;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum MainCommitment {
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
pub enum AstNodeMain{
    Statements(Vec<ImportStatement>),
    Commitments(Vec<MainCommitment>),
}