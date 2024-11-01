use crate::core::applit::entities::bundle::{AppLit, AppLitMode};

impl AppLit {
    pub fn get_mode(&self) -> AppLitMode {
        self.mode.clone()
    }
}