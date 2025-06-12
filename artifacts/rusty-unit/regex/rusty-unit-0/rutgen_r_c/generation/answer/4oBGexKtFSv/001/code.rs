// Answer 0

#[test]
fn test_valid_regex() {
    let result = Regex::new(r"^\d+$");
    assert!(result.is_ok());
}

#[test]
fn test_invalid_regex_syntax() {
    let result = Regex::new(r"(");
    assert!(result.is_err());
    if let Err(Error::Syntax(ref message)) = result {
        assert!(!message.is_empty());
    }
}

#[test]
fn test_empty_regex() {
    let result = Regex::new("");
    assert!(result.is_err());
    if let Err(Error::Syntax(ref message)) = result {
        assert!(!message.is_empty());
    }
}

#[test]
fn test_exceeding_size_limit() {
    let long_regex = "a".repeat(1024 * 1024 + 1); // Example exceeding size limit
    let result = Regex::new(&long_regex);
    assert!(result.is_err());
    if let Err(Error::CompiledTooBig(limit)) = result {
        assert!(limit > 1024 * 1024);
    }
}

#[test]
fn test_valid_regex_with_special_chars() {
    let result = Regex::new(r"\W+");
    assert!(result.is_ok());
}

