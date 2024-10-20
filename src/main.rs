#![allow(unused_imports)]
#![allow(unused_variables)]

use applit::AppLit;

// main.rs still exists to test library
fn main() {
    let app_path = "/Users/marcelarmada-castellon/Documents/ArmadaCore/Repository/applit/mock";

    match AppLit::new(app_path) {
        Ok(app) => match app.cache_and_run() {
            Ok(nodes) => println!("{:#?}", nodes),
            Err(error) => eprintln!("{}", error)
        },
        Err(error) => eprintln!("{}", error)
    }
}
