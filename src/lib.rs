#![allow(unused_variables)]
#![allow(dead_code)]

/// Hint module to give result feedback to user
pub mod feedback;

/// Crate environment variables
pub mod bin;

mod core;

pub fn execute_main(root_dir: &str){
    if let Err(error) = core::execute::main(root_dir) { error.print() }
}