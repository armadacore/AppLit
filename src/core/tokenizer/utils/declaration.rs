use crate::bin::constants;
use crate::core::tokenizer::reader::{TokenReaderSnapshot, TokenReaderStack};
use std::fmt::Debug;

mod expected_token;

mod between_curly_braces;

mod between_single_quotes;

#[derive(Debug, Clone)]
pub enum DeclarationResult {
    ExpectedToken(TokenReaderSnapshot),
    BetweenCurlyBraces(TokenReaderSnapshot),
    BetweenSingleQuotes(TokenReaderSnapshot),
}

pub enum DeclarationState {
    AlreadyFound,
    Found,
    Search
}

pub struct DeclarationValidation {
    stack: Vec<Box<dyn FnMut(TokenReaderSnapshot) -> DeclarationState>>,
}

impl DeclarationValidation {
    pub fn expected_token<F>(&mut self, token: &str, callback: F) -> &mut Self
    where
        F: 'static + FnMut(TokenReaderSnapshot),
    {
        expected_token::validate(self, token, callback);
        self
    }

    pub fn between_curly_braces<F>(&mut self, callback: F) -> &mut Self
    where
        F: 'static + FnMut(TokenReaderSnapshot),
    {
        between_curly_braces::validate(self, callback);
        self
    }

    pub fn between_single_quotes<F>(&mut self, callback: F) -> &mut Self
    where
        F: 'static + FnMut(TokenReaderSnapshot),
    {
        between_single_quotes::validate(self, callback);
        self
    }

    pub fn check<T: Debug + Clone>(&mut self, stack: &mut TokenReaderStack<T>) -> bool {
        loop_over(self, stack)
    }
}

pub fn validate<F>(mut callback: F) -> bool
where
    F: FnMut(&mut DeclarationValidation) -> bool,
{
    callback(&mut DeclarationValidation { stack: vec![] })
}

fn loop_over<T: Debug + Clone>(
    declaration: &mut DeclarationValidation,
    stack: &mut TokenReaderStack<T>,
) -> bool {
    let cb_len = declaration.stack.len();
    let mut result = false;
    
    while let Some(snapshot) = &stack.next() {
        if let Some(token) = &snapshot.token {
            if token.trim().is_empty() {
                continue;
            }
        }

        let mut cb_count = 0;

        for stack_cb in declaration.stack.iter_mut() {
            match stack_cb(snapshot.clone()) { 
                DeclarationState::Found => {
                    result = true; 
                },
                DeclarationState::Search => result = false,
                _ => ()
            }

            cb_count += 1;
        }
        
        if result && cb_count == cb_len {
            return true;
        }

        if let Some(token) = &snapshot.token {
            if token == constants::SEMICOLON_TOKEN {
                break;
            }
        }
    }
    
    false
}
