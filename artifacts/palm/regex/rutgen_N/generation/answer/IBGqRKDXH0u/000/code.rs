// Answer 0

#[derive(Debug)]
struct Error {
    kind: ErrorKind,
}

#[derive(Debug, PartialEq)]
enum ErrorKind {
    Syntax,
    Unsupported,
}

impl Error {
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

#[test]
fn test_error_kind_syntax() {
    let error = Error { kind: ErrorKind::Syntax };
    assert_eq!(error.kind(), &ErrorKind::Syntax);
}

#[test]
fn test_error_kind_unsupported() {
    let error = Error { kind: ErrorKind::Unsupported };
    assert_eq!(error.kind(), &ErrorKind::Unsupported);
}

