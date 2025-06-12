// Answer 0

#[test]
fn test_valid_regex() {
    let result = new(r"\d+");
    assert!(result.is_ok());
}

#[test]
fn test_invalid_regex() {
    let result = new(r"\");
    assert!(result.is_err());
}

#[test]
fn test_empty_regex() {
    let result = new("");
    assert!(result.is_err());
}

#[test]
fn test_whitespace_regex() {
    let result = new(r"\s+");
    assert!(result.is_ok());
}

#[test]
fn test_special_characters_regex() {
    let result = new(r"[a-zA-Z0-9!@#\$%\^&*()]");
    assert!(result.is_ok());
}

#[test]
fn test_nested_parentheses_regex() {
    let result = new(r"((\d{3})-(\d{2})-(\d{4}))");
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_regex_with_mismatched_parentheses() {
    let _ = new(r"((abc)");
}

#[test]
#[should_panic]
fn test_regex_with_unclosed_character_class() {
    let _ = new(r"[abc");
}

