use crate::tokenizer::ast::{AstOperation};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::str::SplitWhitespace;
use crate::feedback::error::{unknown_token, ErrorFeedback};
use crate::tokenizer::tokens::Token;

pub mod import;
pub mod function;

pub fn new(ast_operation: AstOperation) -> Result<Vec<Token>, ErrorFeedback> {
    let mut lines = read_lines(ast_operation);
    let mut ast: Vec<Token> = vec![];

    while let Some(Ok(line)) = lines.next() {
        for token in read_tokens(&line) {
            if import::check(token) {
                ast.push(Token::Import(import::Declaration{
                    specifier: String::from("foobar"),
                    from: String::from("foobar"),
                }));

                continue;
            }

            if function::check() {
                continue;
            }

            return Err(unknown_token(token));
        }
    }

    Ok(ast)
}

fn read_lines(ast_operation: AstOperation) -> Lines<BufReader<File>>{
    let file = File::open(ast_operation.file_path).unwrap();
    let reader = BufReader::new(file);

    reader.lines()
}

fn read_tokens(line: &str) -> SplitWhitespace{
    line.split_whitespace()
}