use crate::token::reader::{TokenReaderLocation, TokenReaderNextLiteral, TokenReaderStack};
use std::cell::Ref;
use std::fmt::Debug;

pub fn from_to(nodes: Ref<Vec<TokenReaderNextLiteral>>) -> TokenReaderLocation {
    let mut location = nodes
        .first()
        .expect("TokenReaderLocation not found")
        .location
        .clone();

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

pub fn get_location<T: Debug>(stack: &TokenReaderStack<T>) -> TokenReaderLocation {
    TokenReaderLocation {
        start: stack.get_start_pos(),
        end: stack.get_end_pos() as isize,
        line_start: stack.get_line_number(),
        line_end: stack.get_line_number() as isize,
    }
}

pub fn update_location_end<T: Debug>(
    stack: &TokenReaderStack<T>,
    location: &mut TokenReaderLocation,
) {
    location.end = stack.get_end_pos() as isize;
    location.line_end = stack.get_line_number() as isize;
}
