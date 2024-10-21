use crate::core::parser::ImportStatement;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MainStatement {
    Import(ImportStatement),
    Id {
        dev_id: String,
        app_id: String
    },
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AstMainNode {
    Statements(Vec<MainStatement>),
}