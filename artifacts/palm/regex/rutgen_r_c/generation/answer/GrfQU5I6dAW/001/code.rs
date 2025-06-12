// Answer 0

#[test]
fn test_escape_empty_string() {
    let input = "";
    let expected = "";
    let result = escape(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_basic_literal() {
    let input = "abc";
    let expected = "abc"; // No special characters to escape
    let result = escape(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_special_characters() {
    let input = ".^$*+?()[{\\|";
    let expected = "\\.\\^\\$\\*\\+\\?\\(\\)\\[\\{\\\\\\|"; // All characters should be escaped
    let result = escape(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_string_with_spaces() {
    let input = "hello world";
    let expected = "hello world"; // No special characters to escape
    let result = escape(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_string_with_mixed_characters() {
    let input = "a.b*c+d?e(f)g{h}|i^j$k";
    let expected = "a\\.b\\*c\\+d\\?e\\(f\\)g\\{h\\}\\|i\\^j\\$k"; // Mixed characters should be properly escaped
    let result = escape(input);
    assert_eq!(result, expected);
}

