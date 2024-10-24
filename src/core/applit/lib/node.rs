use crate::composer::AppLit;
use crate::core::feedback::ErrorCause;
use crate::core::parser::TreeBuilder;
use crate::core::tokenizer::tokenize_file;
use crate::mode::AppLitMode;

pub fn try_create_node_from_source(app_lit: &mut AppLit) -> Result<bool, ErrorCause>{
    if app_lit.mode == AppLitMode::SourceCode {
        let tokens = tokenize_file(&app_lit.entry);
        TreeBuilder::new(tokens).parse(app_lit)?;

        return Ok(true);
    }
    
    Ok(false)
}