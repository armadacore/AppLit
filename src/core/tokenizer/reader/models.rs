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

#[derive(Debug, Clone, PartialEq)]
pub enum TokenDeclaration {
    Keyword(TokenSnapshot),
    Identifier(TokenSnapshot),
    Literal(TokenSnapshot),

    ArgumentOpen(TokenSnapshot),
    ArgumentClose(TokenSnapshot),

    BlockOpen(TokenSnapshot),
    BlockClose(TokenSnapshot),

    StatementAssignment(TokenSnapshot),
    StatementDivider(TokenSnapshot),
    StatementEnd(TokenSnapshot),

    Unknown(TokenSnapshot),
}