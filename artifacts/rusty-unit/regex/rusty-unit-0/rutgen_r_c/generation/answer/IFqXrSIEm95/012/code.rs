// Answer 0

#[test]
fn test_description_group_name_empty() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct TestError {
        kind: ErrorKind,
    }

    impl error::Error for TestError {
        fn description(&self) -> &str {
            use self::ErrorKind::*;
            match self.kind {
                GroupNameEmpty => "empty capture group name",
                _ => "unknown error",
            }
        }
    }

    let error_instance = TestError { kind: ErrorKind::GroupNameEmpty };
    assert_eq!(error_instance.description(), "empty capture group name");
}

#[test]
fn test_description_group_name_duplicate() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct TestError {
        kind: ErrorKind,
    }

    impl error::Error for TestError {
        fn description(&self) -> &str {
            use self::ErrorKind::*;
            match self.kind {
                GroupNameDuplicate { .. } => "duplicate capture group name",
                _ => "unknown error",
            }
        }
    }

    let error_instance = TestError { kind: ErrorKind::GroupNameDuplicate { original: Span { start: Position::new(0), end: Position::new(5) } } };
    assert_eq!(error_instance.description(), "duplicate capture group name");
}

