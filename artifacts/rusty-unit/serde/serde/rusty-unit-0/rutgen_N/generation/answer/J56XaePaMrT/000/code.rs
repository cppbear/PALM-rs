// Answer 0

#[derive(Debug)]
struct CustomError {
    err: String,
}

impl CustomError {
    fn new(err: &str) -> Self {
        CustomError {
            err: err.to_string(),
        }
    }

    fn description(&self) -> &str {
        &self.err
    }
}

#[test]
fn test_description() {
    let error = CustomError::new("This is a test error");
    assert_eq!(error.description(), "This is a test error");
}

#[test]
fn test_empty_description() {
    let error = CustomError::new("");
    assert_eq!(error.description(), "");
}

#[test]
fn test_long_description() {
    let error = CustomError::new("A very long description that exceeds typical lengths and is used for testing purposes.");
    assert_eq!(error.description(), "A very long description that exceeds typical lengths and is used for testing purposes.");
}

