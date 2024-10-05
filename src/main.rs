use applit::tokenize_source;

// main.rs still exists to test library
fn main() {
    let file_path =  "/Users/marcelarmada-castellon/Documents/ArmadaCore/Repository/applit/mock/main.app";
    
    match tokenize_source(file_path) {
        Ok(result) => println!("{:#?}", result),
        Err(error) => eprintln!("{}", error)
    }
}
