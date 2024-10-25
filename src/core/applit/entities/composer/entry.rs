use crate::composer::AppLit;

impl AppLit {
    pub fn get_entry(&self) -> String {
        self.entry.to_string_lossy().to_string()
    }
}