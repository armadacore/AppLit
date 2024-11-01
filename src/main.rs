#![allow(unused_imports)]
#![allow(unused_variables)]

use applit::run_app;
use std::env;

// main.rs still exists to test library
fn main() {
    let project_root = env::current_dir().expect("Failed to get current directory");
    let mock_path = project_root.join("mock");

    run_app(mock_path.to_str().expect("Failed to convert path to string"));
}
