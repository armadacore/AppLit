use crate::bin;

pub const IDENTIFIER_REGEX: &str = r"^[a-zA-Z0-9_]+$";

pub const LITERAL_REGEX: &str = r"'([^']*)'";

pub const REGEX_TOKENS: [&str; 7] = [
    bin::constants::ARGUMENT_OPEN,
    bin::constants::BLOCK_OPEN,
    bin::constants::STATEMENT_ASSIGNMENT,
    bin::constants::STATEMENT_DIVIDER,
    bin::constants::BLOCK_CLOSE,
    bin::constants::ARGUMENT_CLOSE,
    bin::constants::STATEMENT_END,
];