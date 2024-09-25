use crate::bin::constants;
use crate::tokenizer::tokens::lexer::TokenToAst;
use crate::tokenizer::tokens::Token;

#[derive(Debug, Clone)]
pub struct ImportDeclaration {
    pub pos: usize,
    pub end: usize,
    pub line_start: usize,
    pub line_end: usize,
    pub specifier: Vec<String>,
    pub from: Option<String>,

}

pub fn check(t2a: &mut TokenToAst) -> bool {
    if let Some(ref token) = t2a.token {
        if token == constants::IMPORT {
            let mut declaration = ImportDeclaration{
                pos: t2a.pos,
                end: 0,
                line_start: t2a.line_number,
                line_end: 0,
                specifier: vec![],
                from: None
            };

            loop_tokens(&mut declaration, t2a);
            declaration.end = t2a.end;
            declaration.line_end = t2a.line_number;
            t2a.ast.push(Token::Import(declaration));
            
            return true;
        }
    }
    
    false
}

fn loop_tokens(declaration: &mut ImportDeclaration, t2a: &mut TokenToAst){
    while let Some(token) = t2a.next() {
        let mut current_token = token;

        match current_token.as_str() {
            constants::EMPTY | constants::START_CURLY_BRACES | constants::END_CURLY_BRACES => continue,
            constants::FROM => {
                if let Some(from) = t2a.next(){
                    let cleaned = from.replace(constants::SINGLE_QUOTES, "").replace(constants::SEMICOLON, "");
                    declaration.from = Some(cleaned);
                }
                break;
            },
            _ => {
                if current_token.starts_with(constants::START_CURLY_BRACES){
                    current_token.remove(0).to_string();
                }

                if current_token.ends_with(constants::COMMA){
                    current_token.pop();
                }

                if current_token.ends_with(constants::END_CURLY_BRACES){
                    current_token.pop();
                }
            }
        };

        declaration.specifier.push(current_token);
    }
}