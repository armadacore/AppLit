#[derive(Debug, Clone)]
pub struct TokenSnapshot {
    pub location: TokenLocation,
    pub token: String,
}

#[derive(Debug, Clone)]
pub struct TokenLocation {
    pub start: usize,
    pub end: usize,
    pub line: usize,
}

#[derive(Debug, Clone)]
pub enum TokenDeclaration {
    Keyword(TokenSnapshot),
    Identifier(TokenSnapshot),
    Literal(TokenSnapshot),

    ArgumentOpen(TokenSnapshot),
    ArgumentClose(TokenSnapshot),

    BlockOpen(TokenSnapshot),
    BlockClose(TokenSnapshot),

    AssignmentStatement(TokenSnapshot),
    StatementDivider(TokenSnapshot),
    StatementEnd(TokenSnapshot),

    Error(TokenSnapshot),
}