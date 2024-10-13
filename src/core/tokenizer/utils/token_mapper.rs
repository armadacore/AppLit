use crate::bin::constants;
use crate::core::tokenizer::{TokenDeclaration, TokenLocation, TokenSnapshot};
use regex::Regex;

const LITERAL_REGEX: &str = r"'([^']*)'";

const IDENTIFIER_REGEX: &str = r"^[a-zA-Z0-9_]+$";

pub fn match_token(token: &str, line: usize, start: usize, end: usize) -> TokenDeclaration {
    let identifier_regex = Regex::new(IDENTIFIER_REGEX).expect("identifier regex are broken");
    let literal_regex = Regex::new(LITERAL_REGEX).expect("literal regex are broken");
    let token_location = TokenLocation::new(line, start, end);
    let token_snapshot = TokenSnapshot::new(token_location, token.into());

    match token {
        constants::KEYWORD_IMPORT | constants::KEYWORD_FROM => {
            TokenDeclaration::Keyword(token_snapshot)
        }

        constants::ARGUMENT_OPEN => TokenDeclaration::ArgumentOpen(token_snapshot),
        constants::ARGUMENT_CLOSE => TokenDeclaration::ArgumentClose(token_snapshot),

        constants::BLOCK_OPEN => TokenDeclaration::BlockOpen(token_snapshot),
        constants::BLOCK_CLOSE => TokenDeclaration::BlockClose(token_snapshot),

        constants::STATEMENT_ASSIGNMENT => {
            TokenDeclaration::StatementAssignment(token_snapshot)
        }
        constants::STATEMENT_DIVIDER => TokenDeclaration::StatementDivider(token_snapshot),
        constants::STATEMENT_END => TokenDeclaration::StatementEnd(token_snapshot),
        
        commitment_token if commitment_token.starts_with(constants::COMMITMENT_IDENTIFIER) => {
            TokenDeclaration::Commitment(token_snapshot)
        }
        literal_token if literal_regex.is_match(literal_token) => {
            TokenDeclaration::Literal(token_snapshot)
        }
        identifier_token if identifier_regex.is_match(identifier_token) => {
            TokenDeclaration::Identifier(token_snapshot)
        }

        _ => TokenDeclaration::Unknown(token_snapshot),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_snapshot(token: &str) -> TokenSnapshot{
        TokenSnapshot {
            location: TokenLocation{line: 0, start: 0, end: 0},
            token: token.to_string(),
        }
    }

    #[test]
    fn match_keyword_for_import(){
        let input = "import";
        let result = match_token(input, 0, 0, 0);
        let expected = TokenDeclaration::Keyword(create_snapshot(input));

        assert_eq!(expected, result);
    }

    #[test]
    fn match_keyword_for_from(){
        let input = "from";
        let result = match_token(input, 0, 0, 0);
        let expected = TokenDeclaration::Keyword(create_snapshot(input));

        assert_eq!(expected, result);
    }

    #[test]
    fn match_argument_open(){
        let input = "(";
        let result = match_token(input, 0, 0, 0);
        let expected = TokenDeclaration::ArgumentOpen(create_snapshot(input));

        assert_eq!(expected, result);
    }

    #[test]
    fn match_argument_close(){
        let input = ")";
        let result = match_token(input, 0, 0, 0);
        let expected = TokenDeclaration::ArgumentClose(create_snapshot(input));

        assert_eq!(expected, result);
    }

    #[test]
    fn match_block_open(){
        let input = "{";
        let result = match_token(input, 0, 0, 0);
        let expected = TokenDeclaration::BlockOpen(create_snapshot(input));

        assert_eq!(expected, result);
    }

    #[test]
    fn match_block_close(){
        let input = "}";
        let result = match_token(input, 0, 0, 0);
        let expected = TokenDeclaration::BlockClose(create_snapshot(input));

        assert_eq!(expected, result);
    }

    #[test]
    fn match_statement_assignment(){
        let input = ":";
        let result = match_token(input, 0, 0, 0);
        let expected = TokenDeclaration::StatementAssignment(create_snapshot(input));

        assert_eq!(expected, result);
    }

    #[test]
    fn match_statement_divider(){
        let input = ",";
        let result = match_token(input, 0, 0, 0);
        let expected = TokenDeclaration::StatementDivider(create_snapshot(input));

        assert_eq!(expected, result);
    }

    #[test]
    fn match_statement_end(){
        let input = ";";
        let result = match_token(input, 0, 0, 0);
        let expected = TokenDeclaration::StatementEnd(create_snapshot(input));

        assert_eq!(expected, result);
    }
    
    #[test]
    #[ignore]
    fn match_commitment(){
        todo!()
    }
    
    #[test]
    #[ignore]
    fn match_literal(){
        todo!()
    }

    #[test]
    #[ignore]
    fn match_identifier(){
        todo!()
    }

    #[test]
    fn match_unknown(){
        let input = "µ";
        let result = match_token(input, 0, 0, 0);
        let expected = TokenDeclaration::Unknown(create_snapshot(input));

        assert_eq!(expected, result);
    }
}