use crate::bin::constants;
use crate::core::execute::lexer::LexerStack;

#[derive(Debug, Clone)]
pub struct ImportDeclaration {
    pub pos: usize,
    pub end: usize,
    pub line_start: usize,
    pub line_end: usize,
    pub specifier: Vec<String>,
    pub from: Option<String>,

}

pub fn check(stack: &mut LexerStack<super::ModuleToken>) -> bool {
    if let Some(ref token) = stack.get_token() {
        if token == constants::IMPORT {
            let mut declaration = ImportDeclaration{
                pos: stack.get_pos(),
                end: 0,
                line_start: stack.get_line_number(),
                line_end: 0,
                specifier: vec![],
                from: None
            };

            loop_tokens(&mut declaration, stack);
            declaration.end = stack.get_end();
            declaration.line_end = stack.get_line_number();
            stack.ast_add(super::ModuleToken::Import(declaration));

            return true;
        }
    }

    false
}

fn loop_tokens(declaration: &mut ImportDeclaration, stack: &mut LexerStack<super::ModuleToken>){
    
    while let Some(token) = stack.next() {
        let mut current_token = token;

        match current_token.as_str() {
            constants::EMPTY | constants::START_CURLY_BRACES | constants::END_CURLY_BRACES => continue,
            constants::FROM => {
                if let Some(from) = stack.next(){
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