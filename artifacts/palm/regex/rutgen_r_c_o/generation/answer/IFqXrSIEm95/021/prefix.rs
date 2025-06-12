// Answer 0

#[test]
fn test_error_description_escape_hex_invalid_digit() {
    struct TestError {
        kind: ErrorKind,
    }
  
    impl error::Error for TestError {
        fn description(&self) -> &str {
            use self::ErrorKind::*;
            match self.kind {
                ErrorKind::EscapeHexInvalidDigit => "invalid hexadecimal digit",
                _ => unreachable!(),
            }
        }
    }

    let err = TestError { kind: ErrorKind::EscapeHexInvalidDigit };
    let _ = err.description();
}

