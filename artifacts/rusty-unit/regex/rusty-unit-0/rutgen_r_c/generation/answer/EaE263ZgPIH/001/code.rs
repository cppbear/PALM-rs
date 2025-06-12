// Answer 0

#[test]
fn test_escape_empty_string() {
    let result = escape("");
    assert_eq!(result, "");
}

#[test]
fn test_escape_no_meta_characters() {
    let result = escape("abc123");
    assert_eq!(result, "abc123");
}

#[test]
fn test_escape_meta_characters() {
    let result = escape("a.b*c?d+e|f^g$h");
    assert_eq!(result, "a\\.b\\*c\\?d\\+e\\|f\\^g\\$h");
}

#[test]
fn test_escape_special_characters() {
    let result = escape("Hello, World! (test) [example]");
    assert_eq!(result, "Hello, World! \\(test\\) \\[example\\]");
}

#[test]
fn test_escape_single_meta_character() {
    let result = escape(".");
    assert_eq!(result, "\\.");
}

#[test]
fn test_escape_multiple_meta_characters() {
    let result = escape(".*?+|^$");
    assert_eq!(result, "\\.\\*\\?\\+\\|\\^\\$");
}

#[test]
fn test_escape_unicode_characters() {
    let result = escape("你好! $%^&*()");
    assert_eq!(result, "你好! \\$%\\^&\\*\\(\\)");
}

#[test]
fn test_escape_string_with_only_meta_characters() {
    let result = escape(".*+?|()[]{}\\");
    assert_eq!(result, "\\.\\*\\+\\?\\(\\)\\[\\]\\{\\}\\");
}

