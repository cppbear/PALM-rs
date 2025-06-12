// Answer 0

#[derive(Debug)]
struct RegexError {
    pattern: String,
}

impl RegexError {
    pub fn new(pattern: String) -> Self {
        RegexError { pattern }
    }

    pub fn pattern(&self) -> &str {
        &self.pattern
    }
}

#[test]
fn test_pattern_returns_correct_string() {
    let error = RegexError::new("^[a-z]+$".to_string());
    assert_eq!(error.pattern(), "^[a-z]+$");
}

#[test]
fn test_pattern_with_empty_string() {
    let error = RegexError::new("".to_string());
    assert_eq!(error.pattern(), "");
}

#[test]
fn test_pattern_with_special_characters() {
    let error = RegexError::new(".*[0-9]$".to_string());
    assert_eq!(error.pattern(), ".*[0-9]$");
}

