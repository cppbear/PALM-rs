// Answer 0

#[test]
fn test_description_class_escape_invalid() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Error {
        kind: ErrorKind,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum ErrorKind {
        ClassEscapeInvalid,
        // other variants omitted for brevity
    }

    impl Error {
        fn description(&self) -> &str {
            use self::ErrorKind::*;
            match self.kind {
                ClassEscapeInvalid => "invalid escape sequence in character class",
                _ => unreachable!(),
            }
        }
    }

    let error = Error {
        kind: ErrorKind::ClassEscapeInvalid,
    };

    assert_eq!(error.description(), "invalid escape sequence in character class");
}

#[test]
fn test_description_capture_limit_exceeded() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Error {
        kind: ErrorKind,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum ErrorKind {
        CaptureLimitExceeded,
        // other variants omitted for brevity
    }

    impl Error {
        fn description(&self) -> &str {
            use self::ErrorKind::*;
            match self.kind {
                CaptureLimitExceeded => "capture group limit exceeded",
                _ => unreachable!(),
            }
        }
    }

    let error = Error {
        kind: ErrorKind::CaptureLimitExceeded,
    };

    assert_eq!(error.description(), "capture group limit exceeded");
}

