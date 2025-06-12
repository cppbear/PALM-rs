// Answer 0

#[test]
fn test_escape_unrecognized_description() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct TestError {
        kind: ErrorKind,
    }

    impl TestError {
        fn description(&self) -> &str {
            use self::ErrorKind::*;
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
fn test_flag_duplicate_description() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct TestError {
        kind: ErrorKind,
    }

    impl TestError {
        fn description(&self) -> &str {
            use self::ErrorKind::*;
            match self.kind {
                FlagDuplicate { .. } => "duplicate flag",
                _ => unreachable!(),
            }
        }
    }

    let error = TestError {
        kind: ErrorKind::FlagDuplicate { original: Span { start: Position(0), end: Position(1) }},
    };
    assert_eq!(error.description(), "duplicate flag");
}

