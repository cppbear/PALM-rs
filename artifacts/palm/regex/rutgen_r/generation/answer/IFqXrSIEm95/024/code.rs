// Answer 0

#[derive(Debug)]
enum ErrorKind {
    DecimalInvalid,
    // Other variants...
}

struct Error {
    kind: ErrorKind,
}

impl Error {
    fn description(&self) -> &str {
        use self::ErrorKind::*;
        match self.kind {
            DecimalInvalid => "invalid decimal literal",
            _ => unreachable!(),
        }
    }
}

#[test]
fn test_decimal_invalid_description() {
    let error = Error {
        kind: ErrorKind::DecimalInvalid,
    };
    assert_eq!(error.description(), "invalid decimal literal");
}

#[test]
fn test_decimal_invalid_panic() {
    let error = Error {
        kind: ErrorKind::DecimalInvalid,
    };
    let result = error.description();
    assert_eq!(result, "invalid decimal literal");
}

