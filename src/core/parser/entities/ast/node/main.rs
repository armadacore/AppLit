use crate::core::parser::ImportStatement;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

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