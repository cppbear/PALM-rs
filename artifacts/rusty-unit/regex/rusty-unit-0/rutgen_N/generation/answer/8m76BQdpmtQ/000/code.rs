// Answer 0

#[test]
fn test_valid_regex() {
    let result = from_str(r"\d+");
    assert!(result.is_ok());
}

#[test]
fn test_invalid_regex() {
    let result = from_str(r"([a-z");
    assert!(result.is_err());
}

#[test]
fn test_empty_string() {
    let result = from_str("");
    assert!(result.is_err());
}

#[test]
fn test_special_characters() {
    let result = from_str(r"[.*?]");
    assert!(result.is_ok());
}

#[test]
fn test_whitespace_regex() {
    let result = from_str(r"\s+");
    assert!(result.is_ok());
}

