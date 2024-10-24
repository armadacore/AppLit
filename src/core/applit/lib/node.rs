use crate::composer::AppLit;
use crate::core::feedback::ErrorCause;
use crate::core::parser::{AstNode, TreeBuilder};
use crate::core::tokenizer::tokenize_file;
use crate::mode::AppLitMode;

pub fn create_node_from_source_code(app_lit: &AppLit) -> Result<Option<AstNode>, ErrorCause>{
    if app_lit.mode == AppLitMode::SourceCode {
        let tokens = tokenize_file(&app_lit.entry);
        let nodes =TreeBuilder::new(tokens).parse()?;

        return Ok(Some(nodes));
    }
    
    Ok(None)
}