use crate::bin;
use crate::core::tokenizer::reader::TokenDeclaration;
use models::Parser;

mod models;
pub use models::{AstError, AstNode};

mod import;

impl Parser {
    pub fn new(tokens: Vec<TokenDeclaration>) -> Self {
        Parser {
            tokens: tokens.into_iter().peekable(),
        }
    }

    fn parse_program(&mut self) -> Result<AstNode, AstError> {
        let mut statements = Vec::<AstNode>::new();

        while self.tokens.peek().is_some() {
            let statement = self.parse_statement()?;
            statements.push(statement);
        }

        Ok(AstNode::Program { statements })
    }

    fn parse_statement(&mut self) -> Result<AstNode, AstError> {
        let peek = self.tokens.peek();
        
        if peek.is_none() {
            return Err(AstError::UnexpectedEOF);
        }
        
        if let TokenDeclaration::Keyword(snapshot) = peek.unwrap(){
            return match snapshot.token.as_str() { 
                bin::constants::IMPORT_TOKEN => Ok(import::parse(self)?),
                unknown_token => Err(AstError::UnexpectedToken(snapshot.clone())),
            }
        }
        
        panic!("Try to parse on top level for unknown keyword {:#?}", self.tokens.peek().unwrap());
    }
}

pub fn translate_tokens(tokens: Vec<TokenDeclaration>) -> Result<AstNode, AstError> {
    Parser::new(tokens).parse_program()
}
