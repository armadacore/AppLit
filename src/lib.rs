#![allow(unused_variables)]
#![allow(dead_code)]

/// Hint module to give result feedback to user
pub mod feedback;

/// Crate environment variables
pub  mod bin;

mod tokenizer;

pub fn execute_applit(root_dir: &str){
    match tokenizer::ast::look_up(root_dir) {
        Ok(ast) => println!("{:?}", ast),
        Err(error) => error.print(),
    }
}