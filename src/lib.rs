#![allow(unused_variables)]
#![allow(dead_code)]

use crate::core::applit::entities::bundle::AppLit;

mod bin;

mod core;

pub fn run_app(app_path: &str) {
    match AppLit::new(app_path) {
        Ok(mut app) => match app.run() {
            Ok(applit) => println!("{:#?}", applit),
            Err(error) => eprintln!("{}", error),
        },
        Err(error) => eprintln!("{}", error),
    }
}
