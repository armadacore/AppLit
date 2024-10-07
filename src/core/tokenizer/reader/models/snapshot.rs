#[derive(Debug, Clone, PartialEq)]
pub struct TokenLocation {
    pub start: usize,
    pub end: usize,
    pub line: usize,
}

impl TokenLocation {
    pub fn new(start: usize, end: usize, line: usize) -> TokenLocation {
        TokenLocation { start, end, line }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenSnapshot {
    pub location: TokenLocation,
    pub token: String,
}

impl TokenSnapshot {
    pub fn new(location: TokenLocation, token: String) -> Self {
        TokenSnapshot { location, token }
    }
}