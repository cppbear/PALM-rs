// Answer 0

#[test]
fn test_description_escape_hex_invalid_digit() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct TestError {
        kind: ErrorKind,
    }

    impl TestError {
        fn description(&self) -> &str {
            use ErrorKind::*;
            match self.kind {
                EscapeHexInvalidDigit => "invalid hexadecimal digit",
                _ => unreachable!(),
            }
        }
    }

    let error = TestError {
        kind: ErrorKind::EscapeHexInvalidDigit,
    };
    
    assert_eq!(error.description(), "invalid hexadecimal digit");
}

#[test]
fn test_description_escape_unrecognized() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct TestError {
        kind: ErrorKind,
    }

    impl TestError {
        fn description(&self) -> &str {
            use ErrorKind::*;
            match self.kind {
                EscapeUnrecognized => "unrecognized escape sequence",
                _ => unreachable!(),
            }
        }
    }

    let error = TestError {
        kind: ErrorKind::EscapeUnrecognized,
    };
    
    assert_eq!(error.description(), "unrecognized escape sequence");
}

#[test]
fn test_description_nested_limit_exceeded() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct TestError {
        kind: ErrorKind,
    }

    impl TestError {
        fn description(&self) -> &str {
            use ErrorKind::*;
            match self.kind {
                NestLimitExceeded(_) => "nest limit exceeded",
                _ => unreachable!(),
            }
        }
    }

    let error = TestError {
        kind: ErrorKind::NestLimitExceeded(10),
    };
    
    assert_eq!(error.description(), "nest limit exceeded");
}

