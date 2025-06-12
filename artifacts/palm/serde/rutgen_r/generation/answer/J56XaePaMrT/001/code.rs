// Answer 0

#[derive(Debug)]
struct TestError {
    err: String,
}

impl TestError {
    fn new(err: String) -> Self {
        TestError { err }
    }

    fn description(&self) -> &str {
        &self.err
    }
}

#[test]
fn test_description_with_non_empty_error() {
    let error = TestError::new("An error occurred".to_string());
    assert_eq!(error.description(), "An error occurred");
}

#[test]
fn test_description_with_empty_error() {
    let error = TestError::new("".to_string());
    assert_eq!(error.description(), "");
}

#[test]
fn test_description_with_whitespace_error() {
    let error = TestError::new("    ".to_string());
    assert_eq!(error.description(), "    ");
}

