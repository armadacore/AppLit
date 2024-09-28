use crate::feedback::error::ErrorCause;

mod main;

pub fn main(root_dir: &str) -> Result<(), ErrorCause> {
    main::new(root_dir)
}
