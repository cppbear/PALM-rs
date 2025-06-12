// Answer 0

#[test]
fn test_new_valid_regex() {
    let result = regex::new(r"^\d+$");
    assert!(result.is_ok());

    let regex = result.unwrap();
    assert!(regex.is_match("12345"));
    assert!(!regex.is_match("abc"));
}

#[test]
fn test_new_invalid_regex() {
    let result = regex::new(r"[\d+");
    assert!(result.is_err());
}

#[test]
fn test_new_empty_regex() {
    let result = regex::new("");
    assert!(result.is_ok());

    let regex = result.unwrap();
    assert!(regex.is_match(""));
}

#[test]
fn test_new_special_characters() {
    let result = regex::new(r"\s+");
    assert!(result.is_ok());

    let regex = result.unwrap();
    assert!(regex.is_match("    "));
    assert!(!regex.is_match("abc"));
}

