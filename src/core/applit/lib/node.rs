use crate::core::applit::entities::bundle::{AppLit, AppLitMode};
use crate::core::feedback::error::Cause;
use crate::core::parser::node::TreeBuilder;

pub fn try_create_node_from_source(app_lit: &mut AppLit) -> Result<bool, Cause> {
    if app_lit.get_mode() == AppLitMode::ByteCode {
        return Ok(false);
    }

    TreeBuilder::new(app_lit).parse_main()?;

    Ok(true)
}
