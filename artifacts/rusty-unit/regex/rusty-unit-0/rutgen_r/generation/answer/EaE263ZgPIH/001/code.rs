// Answer 0

#[test]
fn test_escape_empty_string() {
    let result = regex_syntax::escape("");
    assert_eq!(result, "");
}

#[test]
fn test_escape_no_meta_characters() {
    let result = regex_syntax::escape("hello");
    assert_eq!(result, "hello");
}

#[test]
fn test_escape_single_meta_character() {
    let result = regex_syntax::escape(".");
    assert_eq!(result, "\\.");
}

#[test]
fn test_escape_multiple_meta_characters() {
    let result = regex_syntax::escape("a.b?c*d{2}");
    assert_eq!(result, "a\\.b\\?c\\*d\\{2\\}");
}

#[test]
fn test_escape_all_meta_characters() {
    let result = regex_syntax::escape("[]{}()^$\\.|?*+");
    assert_eq!(result, "\\[\\]\\{\\}\\(\\)\\^\\$\\\\\\.\\|\\?\\*\\+");
}

#[test]
fn test_escape_string_with_backslash() {
    let result = regex_syntax::escape("C:\\path\\to\\file");
    assert_eq!(result, "C:\\\\path\\\\to\\\\file");
}

#[test]
fn test_escape_unicode_characters() {
    let result = regex_syntax::escape("你好");
    assert_eq!(result, "你好");
}

