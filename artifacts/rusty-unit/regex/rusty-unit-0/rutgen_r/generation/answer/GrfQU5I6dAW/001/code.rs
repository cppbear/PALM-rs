// Answer 0

#[test]
fn test_escape_empty_string() {
    let input = "";
    let expected = "";
    assert_eq!(regex::escape(input), expected);
}

#[test]
fn test_escape_no_special_characters() {
    let input = "abc";
    let expected = "abc";
    assert_eq!(regex::escape(input), expected);
}

#[test]
fn test_escape_special_characters() {
    let input = ".*+?|(){}[]^$\\";
    let expected = "\\.\\*\\+\\?\\|\\(\\)\\{\\}\\[\\]\\^\\$\\\\";
    assert_eq!(regex::escape(input), expected);
}

#[test]
fn test_escape_whitespace_characters() {
    let input = "a b c\t\n\r";
    let expected = "a\\ b\\ c\\t\\n\\r";
    assert_eq!(regex::escape(input), expected);
}

#[test]
fn test_escape_long_string() {
    let input = "This is a very long string containing special characters: .*+?|(){}[]^$\\";
    let expected = "This\\ is\\ a\\ very\\ long\\ string\\ containing\\ special\\ characters:\\ \\.\\*\\+\\?\\|\\(\\)\\{\\}\\[\\]\\^\\$\\\\";
    assert_eq!(regex::escape(input), expected);
}

#[test]
fn test_escape_only_special_characters() {
    let input = ".*+?|(){}[]^$\\";
    let expected = "\\.\\*\\+\\?\\|\\(\\)\\{\\}\\[\\]\\^\\$\\\\";
    assert_eq!(regex::escape(input), expected);
}

