mod statements;

pub mod error;

mod node;
pub use node::{AstNode, TreeBuilder};


// #[cfg(test)]
// mod tests {
//     use crate::core::parser::TreeBuilder;
//     // use crate::core::tokenizer::tests::create_token_declarations;
//     // use std::io::Cursor;
//
//     pub fn create_builder(statement: &str) -> TreeBuilder{
//         // let cursor = Cursor::new(statement);
//         // let mut token_declaration = create_token_declarations(cursor);
//
//         // TreeBuilder::new(token_declaration)
//         todo!()
//     }
// }