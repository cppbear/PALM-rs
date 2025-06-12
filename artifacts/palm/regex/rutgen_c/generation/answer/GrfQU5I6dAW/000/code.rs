// Answer 0

#[test]
fn test_escape_empty_string() {
    let result = escape("");
    assert_eq!(result, "");
}

#[test]
fn test_escape_single_character() {
    let result = escape("a");
    assert_eq!(result, "a");
}

#[test]
fn test_escape_special_characters() {
    let special_chars = r"^$.*+?()[\]{}|\\";
    let escaped = r"\^\$\.\*\+\?\(\)\[\]\{\}\|\\"; // Expected escaped result
    let result = escape(special_chars);
    assert_eq!(result, escaped);
}

#[test]
fn test_escape_mixed_string() {
    let mixed = "hello.*world?^$#";
    let expected = "hello\\.\\*world\\?\\^\\$#"; // Expected escaped result
    let result = escape(mixed);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_whitespace_characters() {
    let result = escape("hello world");
    assert_eq!(result, "hello world");
}

#[test]
fn test_escape_long_string() {
    let long_string = "This string contains special characters: ^, $, ., *, +, ?, (, ), [, ], {, }, |, \\";
    let expected = "This string contains special characters: \\^, \\$, \\.\\*, \\+,
    \\?, \\(, \\), \\[, \\], \\{, \\}, \\|, \\\\"; // Expected escaped result
    let result = escape(long_string);
    assert_eq!(result, expected);
}

