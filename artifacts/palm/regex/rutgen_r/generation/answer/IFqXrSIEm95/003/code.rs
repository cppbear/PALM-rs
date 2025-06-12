// Answer 0

#[test]
fn test_description_unsupported_backreference() {
    struct ErrorKind {
        kind: Kind,
    }

    enum Kind {
        UnsupportedBackreference,
    }

    let error = ErrorKind { kind: Kind::UnsupportedBackreference };
    assert_eq!(error.description(), "backreferences are not supported");
}

