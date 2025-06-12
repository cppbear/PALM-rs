// Answer 0

#[test]
fn test_description_class_unclosed() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Error {
        kind: ErrorKind,
    }

    #[derive(Clone, Copy, Eq, PartialEq)]
    enum ErrorKind {
        ClassUnclosed,
    }

    let error = Error {
        kind: ErrorKind::ClassUnclosed,
    };
    
    assert_eq!(error.description(), "unclosed character class");
}

#[test]
fn test_description_capture_limit_exceeded() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Error {
        kind: ErrorKind,
    }

    #[derive(Clone, Copy, Eq, PartialEq)]
    enum ErrorKind {
        CaptureLimitExceeded,
    }

    let error = Error {
        kind: ErrorKind::CaptureLimitExceeded,
    };
    
    assert_eq!(error.description(), "capture group limit exceeded");
}

#[test]
fn test_description_flag_dangling_negation() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Error {
        kind: ErrorKind,
    }

    #[derive(Clone, Copy, Eq, PartialEq)]
    enum ErrorKind {
        FlagDanglingNegation,
    }

    let error = Error {
        kind: ErrorKind::FlagDanglingNegation,
    };
    
    assert_eq!(error.description(), "dangling flag negation operator");
}

