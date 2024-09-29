use applit::api;

// main.rs still exists to test library
fn main() {
    let root_dir =  "/Users/marcelarmada-castellon/Documents/ArmadaCore/Repository/applit/mock";
    if let Err(error) = api::main(root_dir){
        eprintln!("{}", error);
    }
}
