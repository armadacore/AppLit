use crate::core::feedback::error::Cause;
use crate::core::parser::TreeBuilder;
use crate::mode::AppLitMode;
use crate::AppLit;

pub fn try_create_node_from_source(app_lit: &mut AppLit) -> Result<bool, Cause> {
    if app_lit.get_mode() == AppLitMode::ByteCode {
        return Ok(false);
    }

    TreeBuilder::new(app_lit).parse_main()?;

    Ok(true)
}
