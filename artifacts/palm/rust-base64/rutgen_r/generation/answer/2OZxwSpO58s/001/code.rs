// Answer 0

#[test]
fn test_invalid_padding() {
    struct DummyError {
        kind: DummyErrorKind,
    }

    enum DummyErrorKind {
        InvalidPadding,
    }

    impl std::fmt::Debug for DummyError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self.kind {
                DummyErrorKind::InvalidPadding => write!(f, "Invalid padding"),
            }
        }
    }

    let error = DummyError {
        kind: DummyErrorKind::InvalidPadding,
    };

    let result = format!("{:?}", error);
    assert_eq!(result, "Invalid padding");
}

#[test]
#[should_panic(expected = "Invalid padding")]
fn test_invalid_padding_should_panic() {
    struct DummyError {
        kind: DummyErrorKind,
    }

    enum DummyErrorKind {
        InvalidPadding,
    }

    impl std::fmt::Debug for DummyError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self.kind {
                DummyErrorKind::InvalidPadding => write!(f, "Invalid padding"),
            }
        }
    }

    let error = DummyError {
        kind: DummyErrorKind::InvalidPadding,
    };

    panic!("{:?}", error);
}

