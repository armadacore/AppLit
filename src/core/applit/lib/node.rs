use crate::composer::AppLit;
use crate::core::feedback::ErrorCause;
use crate::core::parser::{module_tree_builder, AstNode};
use crate::core::tokenizer::tokenize_file;
use crate::mode::AppLitMode;

pub fn create_node_from_source_code(app_lit: &AppLit) -> Result<Option<AstNode>, ErrorCause>{
    if app_lit.mode == AppLitMode::SourceCode {
        let tokens = tokenize_file(&app_lit.entry);
        let nodes = module_tree_builder(tokens)?;

        return Ok(Some(nodes));
    }
    
    Ok(None)
}