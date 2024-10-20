use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TokenLocation {
    pub start: usize,
    pub end: usize,
    pub line: usize,
}

impl TokenLocation {
    pub fn new(line: usize, start: usize, end: usize) -> TokenLocation {
        TokenLocation { start, end, line }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TokenSnapshot {
    pub location: TokenLocation,
    pub token: String,
}

impl TokenSnapshot {
    pub fn new(location: TokenLocation, token: String) -> Self {
        TokenSnapshot { location, token }
    }
}