use applit::bin::{mock_constants};
use applit::{execute_main};

// main.rs still exists to test library
fn main(){
    execute_main(mock_constants::ROOT_DIR)
}