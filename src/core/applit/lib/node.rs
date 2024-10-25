use crate::composer::AppLit;
use crate::core::feedback::ErrorCause;
use crate::core::parser::TreeBuilder;
use crate::core::tokenizer::tokenize_file;
use crate::mode::AppLitMode;

pub fn try_create_node_from_source(app_lit: &mut AppLit) -> Result<bool, ErrorCause> {
    if app_lit.get_mode() == AppLitMode::SourceCode {
        let mut tokens = tokenize_file(&app_lit.get_entry());
        TreeBuilder::new(app_lit).parse_app(&mut tokens)?;

        return Ok(true);
    }

    Ok(false)
}
