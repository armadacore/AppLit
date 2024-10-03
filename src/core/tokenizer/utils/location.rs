use crate::core::tokenizer::reader::{TokenReaderLocation, TokenReaderSnapshot, TokenReaderStack};
use std::cell::Ref;
use std::fmt::Debug;

pub fn from_to(nodes: Ref<Vec<TokenReaderSnapshot>>) -> TokenReaderLocation {
    let mut location = nodes
        .first()
        .expect("TokenReaderLocation not found")
        .location.clone();

    if nodes.len() > 1 {
        if let Some(last) = nodes.last() {
            location.end = last.location.end;
            location.line_end = last.location.line_end;
        } else {
            panic!("Location range cannot be resolved");
        }
    }

    location
}

pub fn get_location<T: Debug + Clone>(stack: &TokenReaderStack<T>) -> TokenReaderLocation {
    TokenReaderLocation {
        start: stack.get_start_pos(),
        end: stack.get_end_pos(),
        line_start: stack.get_line_number(),
        line_end: stack.get_line_number(),
    }
}

pub fn update_location_end<T: Debug + Clone>(
    stack: &TokenReaderStack<T>,
    location: &mut TokenReaderLocation,
) {
    location.end = stack.get_end_pos();
    location.line_end = stack.get_line_number();
}
