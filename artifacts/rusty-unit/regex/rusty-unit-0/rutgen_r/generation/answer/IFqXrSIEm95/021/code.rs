// Answer 0

#[derive(Debug)]
struct ErrorKind {
    kind: ErrorKindVariant,
}

#[derive(Debug)]
enum ErrorKindVariant {
    EscapeHexInvalidDigit,
}

impl ErrorKind {
    fn description(&self) -> &str {
        use self::ErrorKindVariant::*;
        match self.kind {
            EscapeHexInvalidDigit => "invalid hexadecimal digit",
            _ => unreachable!(),
        }
    }
}

#[test]
fn test_description_escape_hex_invalid_digit() {
    let error = ErrorKind {
        kind: ErrorKindVariant::EscapeHexInvalidDigit,
    };
    assert_eq!(error.description(), "invalid hexadecimal digit");
}

