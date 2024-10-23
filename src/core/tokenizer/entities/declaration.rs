use crate::core::tokenizer::TokenSnapshot;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenDeclaration {
    Commitment(TokenSnapshot),
    
    Keyword(TokenSnapshot),
    Identifier(TokenSnapshot),
    Literal(TokenSnapshot),
    
    Separator(TokenSnapshot),

    ArgumentOpen(TokenSnapshot),
    ArgumentClose(TokenSnapshot),

    BlockOpen(TokenSnapshot),
    BlockClose(TokenSnapshot),

    IndicesOpen(TokenSnapshot),
    IndicesClose(TokenSnapshot),

    StatementAssignment(TokenSnapshot),
    StatementDivider(TokenSnapshot),
    StatementEnd(TokenSnapshot),

    Unknown(TokenSnapshot),
}

impl TokenDeclaration {
    pub fn extract_snapshot(&self) -> TokenSnapshot {
        match self {
            TokenDeclaration::Commitment(snapshot) => snapshot.clone(),
            TokenDeclaration::Keyword(snapshot) => snapshot.clone(),
            TokenDeclaration::Identifier(snapshot) => snapshot.clone(),
            TokenDeclaration::Literal(snapshot) => snapshot.clone(),
            TokenDeclaration::Separator(snapshot) => snapshot.clone(),
            TokenDeclaration::ArgumentOpen(snapshot) => snapshot.clone(),
            TokenDeclaration::ArgumentClose(snapshot) => snapshot.clone(),
            TokenDeclaration::BlockOpen(snapshot) => snapshot.clone(),
            TokenDeclaration::BlockClose(snapshot) => snapshot.clone(),
            TokenDeclaration::IndicesOpen(snapshot) => snapshot.clone(),
            TokenDeclaration::IndicesClose(snapshot) => snapshot.clone(),
            TokenDeclaration::StatementAssignment(snapshot) => snapshot.clone(),
            TokenDeclaration::StatementDivider(snapshot) => snapshot.clone(),
            TokenDeclaration::StatementEnd(snapshot) => snapshot.clone(),
            TokenDeclaration::Unknown(snapshot) => snapshot.clone(),
        }
    }
}