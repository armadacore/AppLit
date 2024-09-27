use crate::feedback::error::ErrorFeedback;

mod token_reader;

mod token;

mod main;

pub fn main(root_dir: &str) -> Result<(), ErrorFeedback>{
    main::new(root_dir)
}