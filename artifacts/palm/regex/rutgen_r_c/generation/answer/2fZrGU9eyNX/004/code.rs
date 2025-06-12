// Answer 0

#[test]
fn test_auxiliary_span_flag_duplicate() {
    struct TestError {
        kind: ErrorKind,
    }

    impl Error {
        fn kind(&self) -> &ErrorKind {
            &self.kind
        }
    }

    let span = Span {
        start: Position { offset: 0 },
        end: Position { offset: 5 },
    };

    let test_error = Error {
        kind: ErrorKind::FlagDuplicate { original: span.clone() },
    };

    assert_eq!(test_error.auxiliary_span(), Some(&span));
}

#[test]
fn test_auxiliary_span_flag_repeated_negation() {
    struct TestError {
        kind: ErrorKind,
    }

    impl Error {
        fn kind(&self) -> &ErrorKind {
            &self.kind
        }
    }

    let span = Span {
        start: Position { offset: 3 },
        end: Position { offset: 6 },
    };

    let test_error = Error {
        kind: ErrorKind::FlagRepeatedNegation { original: span.clone() },
    };

    assert_eq!(test_error.auxiliary_span(), Some(&span));
}

#[test]
fn test_auxiliary_span_group_name_duplicate() {
    struct TestError {
        kind: ErrorKind,
    }

    impl Error {
        fn kind(&self) -> &ErrorKind {
            &self.kind
        }
    }

    let span = Span {
        start: Position { offset: 2 },
        end: Position { offset: 4 },
    };

    let test_error = Error {
        kind: ErrorKind::GroupNameDuplicate { original: span.clone() },
    };

    assert_eq!(test_error.auxiliary_span(), Some(&span));
}

#[test]
fn test_auxiliary_span_no_auxiliary() {
    struct TestError {
        kind: ErrorKind,
    }

    impl Error {
        fn kind(&self) -> &ErrorKind {
            &self.kind
        }
    }

    let test_error = Error {
        kind: ErrorKind::ClassEscapeInvalid,
    };

    assert_eq!(test_error.auxiliary_span(), None);
}

