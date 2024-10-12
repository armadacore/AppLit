use crate::bin::constants;
use regex::Regex;

pub fn split_line(line: &str) -> Vec<String> {
    let regex_tokens = constants::REGEX_TOKENS_CONDITION.join("");
    let regex_pattern = format!(r#"'(?:\\'|[^'])*'|\w+|[{}]|\s"#, regex_tokens);
    let regexp = Regex::new(&regex_pattern).unwrap();
    let result: Vec<String> = regexp
        .find_iter(line)
        .map(|res| res.as_str().to_string())
        .collect();

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn split_by_tokens_whitespaces_and_literals() {
        let line = "import {pi,co} from 'applit';";
        let result = split_line(line);
        let expected = vec![
            "import",
            " ",
            "{",
            "pi",
            ",",
            "co",
            "}",
            " ",
            "from",
            " ",
            "'applit'",
            ";",
        ];

        assert_eq!(expected, result);
    }

    #[test]
    fn split_by_literals_ignore_tokens_inside() {
        let line = "import {pi,co} from '{\\'applit)';";
        let result = split_line(line);
        let expected = vec![
            "import",
            " ",
            "{",
            "pi",
            ",",
            "co",
            "}",
            " ",
            "from",
            " ",
            "'{\\'applit)'",
            ";",
        ];

        assert_eq!(expected, result);
    }
}