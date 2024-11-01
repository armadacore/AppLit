use crate::mode::AppLitMode;
use crate::AppLit;

impl AppLit {
    pub fn get_mode(&self) -> AppLitMode {
        self.mode.clone()
    }
}