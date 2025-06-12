// Answer 0

#[test]
fn test_from_str_valid_regex() {
    let result = Regex::from_str("a*b+");
    assert!(result.is_ok());
}

#[test]
fn test_from_str_invalid_regex() {
    let result = Regex::from_str("[a-");
    assert!(result.is_err());
}

#[test]
fn test_from_str_empty_string() {
    let result = Regex::from_str("");
    assert!(result.is_err());
}

#[test]
fn test_from_str_single_character() {
    let result = Regex::from_str("a");
    assert!(result.is_ok());
}

#[test]
fn test_from_str_special_characters() {
    let result = Regex::from_str("\\d+");
    assert!(result.is_ok());
}

