use crate::core::execute::token_reader::{TokenReaderLocation, TokenReaderNextLiteral};
use std::cell::Ref;

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
