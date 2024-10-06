use crate::core::tokenizer::reader::utils::split_line;
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
