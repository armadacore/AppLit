#[derive(Debug)]
pub struct Declaration {
    pub specifier: String,
    pub from: String,
}

pub fn check(token: &str) -> bool {
    token == "import"
}