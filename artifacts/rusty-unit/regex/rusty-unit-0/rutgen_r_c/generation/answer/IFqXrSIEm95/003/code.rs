// Answer 0

#[test]
fn test_description_unsupported_backreference() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Error {
        kind: ErrorKind,
    }

    #[derive(Clone, Copy, Eq, PartialEq)]
    enum ErrorKind {
        UnsupportedBackreference,
    }

    let error = Error {
        kind: ErrorKind::UnsupportedBackreference,
    };

    assert_eq!(error.description(), "backreferences are not supported");
}

