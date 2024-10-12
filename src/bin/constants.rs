use crate::bin;

pub const MAIN_APP_FILE: &str = "main.app";

pub const MAIN_APPLIT_FILE: &str = "main.applit";

pub const EMPTY_STRING: &str = "";

pub const SPACE_STRING: &str = " ";

pub const STATEMENT_DIVIDER: &str = ",";

pub const STATEMENT_END: &str = ";";

pub const LITERAL: &str = "'";

pub const BLOCK_OPEN: &str = "{";

pub const STATEMENT_ASSIGNMENT: &str = ":";

pub const BLOCK_CLOSE: &str = "}";

pub const ARGUMENT_OPEN: &str = "(";

pub const ARGUMENT_CLOSE: &str = ")";

pub const KEYWORD_IMPORT: &str = "import";

pub const KEYWORD_FROM: &str = "from";

pub const REGEX_TOKENS_CONDITION: [&str; 7] = [
    ARGUMENT_OPEN,
    BLOCK_OPEN,
    STATEMENT_ASSIGNMENT,
    STATEMENT_DIVIDER,
    BLOCK_CLOSE,
    ARGUMENT_CLOSE,
    STATEMENT_END,
];