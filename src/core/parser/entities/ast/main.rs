use crate::core::parser::ImportStatement;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum MainStatement {
    Import(ImportStatement),
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
    Statements(Vec<MainStatement>),
}