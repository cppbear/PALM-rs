// Answer 0

#[derive(Debug)]
struct CustomError {
    kind: ErrorKind,
}

#[derive(Debug)]
enum ErrorKind {
    SyntaxError,
    TimeoutError,
}

impl CustomError {
    pub fn new(kind: ErrorKind) -> Self {
        CustomError { kind }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

#[test]
fn test_kind_returns_error_kind() {
    let error = CustomError::new(ErrorKind::SyntaxError);
    assert_eq!(format!("{:?}", error.kind()), "SyntaxError");

    let timeout_error = CustomError::new(ErrorKind::TimeoutError);
    assert_eq!(format!("{:?}", timeout_error.kind()), "TimeoutError");
}

