// Answer 0

#[test]
fn test_fmt_unsupported_look_around() {
    use std::fmt::Formatter;

    struct ErrorKindWrapper {
        kind: ErrorKind,
    }

    impl fmt::Display for ErrorKindWrapper {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            use self::ErrorKind::*;
            match self.kind {
                UnsupportedLookAround => {
                    write!(f, "look-around, including look-ahead and look-behind, is not supported")
                }
                _ => write!(f, "not an unsupported look-around error"),
            }
        }
    }

    let error = ErrorKindWrapper {
        kind: ErrorKind::UnsupportedLookAround,
    };

    let mut buffer = String::new();
    let result = error.fmt(&mut buffer);

    assert!(result.is_ok());
    assert_eq!(buffer, "look-around, including look-ahead and look-behind, is not supported");
}

#[test]
fn test_fmt_other_error_kinds() {
    use std::fmt::Formatter;

    struct ErrorKindWrapper {
        kind: ErrorKind,
    }

    impl fmt::Display for ErrorKindWrapper {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            use self::ErrorKind::*;
            match self.kind {
                UnsupportedLookAround => {
                    write!(f, "look-around, including look-ahead and look-behind, is not supported")
                }
                // Test other error kinds here if needed
                _ => write!(f, "not an unsupported look-around error"),
            }
        }
    }

    let error = ErrorKindWrapper {
        kind: ErrorKind::ClassEscapeInvalid,
    };

    let mut buffer = String::new();
    let result = error.fmt(&mut buffer);

    assert!(result.is_ok());
    assert_eq!(buffer, "not an unsupported look-around error");
}

