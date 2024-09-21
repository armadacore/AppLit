use applit::tokenizer;


const ROOT_DIR: &str = "/Users/marcelarmada-castellon/Documents/ArmadaCore/Test/applit";

fn main(){
    match tokenizer::ast::create(ROOT_DIR) {
        Ok(ast) => println!("{:?}", ast),
        Err(error) => error.print(),
    }
}