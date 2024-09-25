use applit::bin::{mock_constants};
use applit::{execute_applit};

// main.rs still exists to test library
fn main(){
    execute_applit(mock_constants::ROOT_DIR)
}