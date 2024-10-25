use crate::composer::AppLit;
use crate::mode::AppLitMode;

impl AppLit {
    pub fn get_mode(&self) -> AppLitMode {
        self.mode.clone()
    }
}