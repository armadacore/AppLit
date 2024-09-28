use crate::feedback::error::ErrorCause;

mod token_reader;

mod token;

mod main;

pub fn main(root_dir: &str) -> Result<(), ErrorCause> {
    main::new(root_dir)
}
