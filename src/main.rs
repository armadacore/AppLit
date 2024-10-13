#![allow(unused_imports)]

use applit::{parse_module, tokenize_source};

// main.rs still exists to test library
fn main() {
    let file_path =  "/Users/marcelarmada-castellon/Documents/ArmadaCore/Repository/applit/mock/main.app";
    
    match tokenize_source(file_path) {
        Ok(tokens) => println!("{:#?}", parse_module(tokens)),
        Err(error) => eprintln!("{}", error)
    }
}
