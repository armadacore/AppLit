use applit::tokenizer;
use applit::bin::{constants};

fn main(){
    match tokenizer::ast::create(constants::ROOT_DIR) {
        Ok(ast) => println!("{:?}", ast),
        Err(error) => error.print(),
    }
}