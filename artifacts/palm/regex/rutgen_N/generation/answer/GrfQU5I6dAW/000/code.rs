// Answer 0

#[test]
fn test_escape_empty_string() {
    let result = regex::escape("");
    assert_eq!(result, "");
}

#[test]
fn test_escape_no_meta_characters() {
    let result = regex::escape("hello world");
    assert_eq!(result, "hello world");
}

#[test]
fn test_escape_with_meta_characters() {
    let result = regex::escape("abc?*+|()[]{}\\.");
    assert_eq!(result, "abc\\?\\*\\+\\|\\(\\)\\[\\]\\{\\}\\\\\\.");
}

#[test]
fn test_escape_only_meta_characters() {
    let result = regex::escape(".*+?^$()|{}[]\\");
    assert_eq!(result, "\\.\\*\\+\\?\\^\\$\\(\\)\\|\\{\\}\\[\\]\\\\");
}

#[test]
fn test_escape_special_character() {
    let result = regex::escape("hello\\world");
    assert_eq!(result, "hello\\\\world");
}

