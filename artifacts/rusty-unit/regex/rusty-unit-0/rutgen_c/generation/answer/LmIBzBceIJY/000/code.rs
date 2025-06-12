// Answer 0

#[test]
fn test_new_valid_regex() {
    let result = Regex::new(r"^\d+$");
    assert!(result.is_ok());
}

#[test]
fn test_new_invalid_regex() {
    let result = Regex::new(r"[a-z+");
    assert!(result.is_err());
    if let Err(Error::Syntax(ref err_msg)) = result {
        assert!(!err_msg.is_empty());
    }
}

#[test]
fn test_new_empty_regex() {
    let result = Regex::new("");
    assert!(result.is_ok());
}

