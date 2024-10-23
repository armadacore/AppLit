pub const SOURCE_CODE_FILE: &str = "main.app";

pub const BINARY_CODE_FILE: &str = "main.applit";

pub const EMPTY_STRING: &str = "";

pub const SPACE_STRING: &str = " ";

pub const SEPARATOR: &str = ".";

pub const COMMITMENT_IDENTIFIER: &str = "@";

pub const COMMITMENT_ID: &str = "@id";

pub const COMMITMENT_ICON: &str = "@icon";

pub const COMMITMENT_NAME: &str = "@name";

pub const COMMITMENT_VERSION: &str = "@version";

pub const COMMITMENT_DESCRIPTION: &str = "@description";

pub const COMMITMENT_LINK: &str = "@link";

pub const COMMITMENT_DOMAIN: &str = "@domain";

pub const STATEMENT_DIVIDER: &str = ",";

pub const STATEMENT_END: &str = ";";

pub const STATEMENT_LITERAL: &str = "'";

pub const STATEMENT_BLOCK_OPEN: &str = "{";

pub const STATEMENT_ASSIGNMENT: &str = ":";

pub const STATEMENT_BLOCK_CLOSE: &str = "}";

pub const STATEMENT_ARGUMENT_OPEN: &str = "(";

pub const STATEMENT_ARGUMENT_CLOSE: &str = ")";

pub const STATEMENT_INDICES_OPEN: &str = "[";

pub const STATEMENT_INDICES_CLOSE: &str = "]";

pub const KEYWORD_IMPORT: &str = "import";

pub const KEYWORD_FROM: &str = "from";

pub const KEYWORD_FUNCTION: &str = "function";

pub const REGEX_TOKENS_CONDITION: [&str; 10] = [
    SEPARATOR,
    STATEMENT_ARGUMENT_OPEN,
    STATEMENT_ARGUMENT_CLOSE,
    STATEMENT_BLOCK_OPEN,
    STATEMENT_BLOCK_CLOSE,
    STATEMENT_INDICES_OPEN,
    STATEMENT_INDICES_CLOSE,
    STATEMENT_ASSIGNMENT,
    STATEMENT_DIVIDER,
    STATEMENT_END,
];