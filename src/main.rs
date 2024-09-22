use applit::tokenizer;
use applit::bin::{mock_constants};

fn main(){
    match tokenizer::ast::look_up(mock_constants::ROOT_DIR) {
        Ok(ast) => println!("{:?}", ast),
        Err(error) => error.print(),
    }
}