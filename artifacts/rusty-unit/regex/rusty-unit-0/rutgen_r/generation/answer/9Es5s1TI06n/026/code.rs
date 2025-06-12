// Answer 0

#[test]
fn test_fmt_class_unclosed() {
    use std::fmt;

    struct Error {
        kind: ErrorKind,
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self.kind {
                ErrorKind::ClassUnclosed => {
                    write!(f, "unclosed character class")
                }
                _ => unreachable!(),
            }
        }
    }

    enum ErrorKind {
        ClassUnclosed,
    }

    let error = Error {
        kind: ErrorKind::ClassUnclosed,
    };

    let formatted = format!("{}", error);
    assert_eq!(formatted, "unclosed character class");
}

#[test]
fn test_fmt_capture_limit_exceeded() {
    use std::fmt;

    struct Error {
        kind: ErrorKind,
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self.kind {
                ErrorKind::CaptureLimitExceeded => {
                    write!(f, "exceeded the maximum number of capturing groups ({})", ::std::u32::MAX)
                }
                _ => unreachable!(),
            }
        }
    }

    enum ErrorKind {
        CaptureLimitExceeded,
    }

    let error = Error {
        kind: ErrorKind::CaptureLimitExceeded,
    };

    let formatted = format!("{}", error);
    assert_eq!(formatted, format!("exceeded the maximum number of capturing groups ({})", ::std::u32::MAX));
}

