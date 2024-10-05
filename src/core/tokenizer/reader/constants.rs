use crate::bin;

pub const IDENTIFIER_REGEX: &str = r"^[a-zA-Z0-9_]+$";

pub const LITERAL_REGEX: &str = r"'([^']*)'";

pub const REGEX_TOKENS: [&str; 7] = [
    bin::constants::LEFT_ROUND_BRACKETS_TOKEN,
    bin::constants::LEFT_CURLY_BRACES_TOKEN,
    bin::constants::COLON_TOKEN,
    bin::constants::COMMA_TOKEN,
    bin::constants::RIGHT_CURLY_BRACES_TOKEN,
    bin::constants::RIGHT_ROUND_BRACKETS_TOKEN,
    bin::constants::SEMICOLON_TOKEN,
];