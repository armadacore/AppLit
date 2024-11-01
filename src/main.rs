#![allow(unused_imports)]
#![allow(unused_variables)]

use applit::AppLit;

// main.rs still exists to test library
fn main() {
    let app_path = "/Users/marcelarmada-castellon/Documents/ArmadaCore/Repository/applit/mock";

    match AppLit::new(app_path) {
        Ok(mut app) => match app.run() {
            Ok(applit) => println!("{:#?}", applit),
            Err(error) => eprintln!("{}", error)
        },
        Err(error) => eprintln!("{}", error)
    }
}
