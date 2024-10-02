use std::collections::HashMap;
use crate::core::feedback::error::ErrorCause;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;
use std::vec;

mod next;

mod next_literal;

mod syntax_error;

pub type TokenReaderNodes<T> = Vec<T>;

#[derive(Debug)]
pub struct TokenReaderStack<T: Debug + Clone> {
    lines: Lines<BufReader<File>>,
    line: Option<String>,
    start: usize,
    end: usize,
    line_number: usize,
    tokens: Vec<String>,
    token: Option<String>,
    errors: Vec<ErrorCause>,
    references: HashMap<String, HashMap<String, String>>,
    declarations: Vec<T>,
}

#[derive(Debug, Clone)]
pub struct TokenReaderLocation {
    pub start: usize,
    pub end: usize,
    pub line_start: usize,
    pub line_end: usize,
}

#[derive(Debug, Clone)]
pub struct TokenReaderNextLiteral {
    pub location: TokenReaderLocation,
    pub prev_token: Option<String>,
    pub token: String,
}

impl<T: Debug + Clone> TokenReaderStack<T> {
    pub fn get_start_pos(&self) -> usize {
        self.start
    }

    pub fn get_end_pos(&self) -> usize {
        self.end
    }

    pub fn get_line_number(&self) -> usize {
        self.line_number
    }

    pub fn get_token(&self) -> Option<String> {
        self.token.clone()
    }

    pub fn push_declaration(&mut self, value: T) {
        self.declarations.push(value);
    }

    pub fn get_declaration(&self) -> Vec<T> {
        self.declarations.clone()
    }

    pub fn next(&mut self) -> Option<String> {
        next::token(self)
    }

    pub fn next_literal(&mut self) -> Option<TokenReaderNextLiteral> {
        next_literal::token(self)
    }

    pub fn syntax_error(&mut self, location: TokenReaderLocation, kind: &str) {
        syntax_error::declaration_report(self, location, kind)
    }
}

pub fn run<T: Debug + Clone, F>(
    file_path: &Path,
    callback: F,
) -> Result<TokenReaderStack<T>, ErrorCause>
where
    F: FnMut(&mut TokenReaderStack<T>) -> bool,
{
    new(file_path, callback)
}

pub fn run_tokens<T: Debug + Clone, F>(
    file_path: &Path,
    tokens: &mut [F],
) -> Result<TokenReaderStack<T>, ErrorCause>
where
    F: FnMut(&mut TokenReaderStack<T>) -> bool,
{
    new(file_path, |stack| {
        let mut token_classified = false;
        if let Some(token) = stack.get_token() {
            for cb in tokens.iter_mut() {
                token_classified = cb(stack);

                if token_classified {
                    continue;
                }
            }
        }

        token_classified
    })
}

fn new<T: Debug + Clone, F>(
    file_path: &Path,
    mut callback: F,
) -> Result<TokenReaderStack<T>, ErrorCause>
where
    F: FnMut(&mut TokenReaderStack<T>) -> bool,
{
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut stack: TokenReaderStack<T> = TokenReaderStack {
        lines: reader.lines(),
        line: None,
        start: 0,
        end: 0,
        line_number: 0,
        tokens: vec![],
        token: None,
        errors: vec![],
        references: HashMap::new(),
        declarations: vec![],
    };

    while let Some(token) = &stack.next() {
        if !callback(&mut stack) {
            syntax_error::report(&mut stack)
        }
    }

    Ok(stack)
}
