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

impl TokenDeclaration {
    pub fn extract_snapshot(&self) -> TokenSnapshot {
        match self {
            TokenDeclaration::Keyword(snapshot) => snapshot.clone(),
            TokenDeclaration::Identifier(snapshot) => snapshot.clone(),
            TokenDeclaration::Literal(snapshot) => snapshot.clone(),
            TokenDeclaration::ArgumentOpen(snapshot) => snapshot.clone(),
            TokenDeclaration::ArgumentClose(snapshot) => snapshot.clone(),
            TokenDeclaration::BlockOpen(snapshot) => snapshot.clone(),
            TokenDeclaration::BlockClose(snapshot) => snapshot.clone(),
            TokenDeclaration::StatementAssignment(snapshot) => snapshot.clone(),
            TokenDeclaration::StatementDivider(snapshot) => snapshot.clone(),
            TokenDeclaration::StatementEnd(snapshot) => snapshot.clone(),
            TokenDeclaration::Unknown(snapshot) => snapshot.clone(),
        }
    }
}