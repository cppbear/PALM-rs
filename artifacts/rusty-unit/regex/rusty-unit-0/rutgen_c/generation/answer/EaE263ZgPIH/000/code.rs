// Answer 0

#[test]
fn test_escape_empty_string() {
    let input = "";
    let expected = "";
    let result = escape(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_no_meta_characters() {
    let input = "abc";
    let expected = "abc";
    let result = escape(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_single_meta_character() {
    let input = ".";
    let expected = "\\.";
    let result = escape(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_multiple_meta_characters() {
    let input = "a.b*c?d+e^f|g(h)[]{}";
    let expected = "a\\.b\\*c\\?d\\+e\\^f\\|g\\(h\\)\\[\\]\\{\\}";
    let result = escape(input);
    assert_eq!(result, expected);
}

#[test]
fn test_escape_all_meta_characters() {
    let input = r"^$.*+?{}()|[]";
    let expected = r"\^\$\.\*\+\?\{\}\(\)\|\\\[\]";
    let result = escape(input);
    assert_eq!(result, expected);
}

