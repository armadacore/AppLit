#![allow(unused_imports)]

use applit::{parse_source, tokenize_source};

// main.rs still exists to test library
fn main() {
    let file_path =  "/Users/marcelarmada-castellon/Documents/ArmadaCore/Repository/applit/mock/main.app";
    
    match tokenize_source(file_path) {
        // Ok(tokens) => println!("{:#?}", tokens),
        Ok(tokens) => println!("{:#?}", parse_source(tokens)),
        Err(error) => eprintln!("{}", error)
    }
}
