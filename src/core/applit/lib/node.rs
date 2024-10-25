use crate::bundle::AppLit;
use crate::core::feedback::ErrorCause;
use crate::core::parser::TreeBuilder;
use crate::mode::AppLitMode;

pub fn try_create_node_from_source(app_lit: &mut AppLit) -> Result<bool, ErrorCause> {
    if app_lit.get_mode() == AppLitMode::ByteCode {
        return Ok(false);
    }

    TreeBuilder::new(app_lit).parse_app()?;

    Ok(true)
}
