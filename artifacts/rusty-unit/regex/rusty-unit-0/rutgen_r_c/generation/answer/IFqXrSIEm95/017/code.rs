// Answer 0

#[test]
fn test_description_flag_duplicate() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct TestError {
        kind: ErrorKind,
    }

    impl TestError {
        fn description(&self) -> &str {
            use self::ErrorKind::*;
            match self.kind {
                FlagDuplicate{..} => "duplicate flag",
                _ => unreachable!(),
            }
        }
    }

    let error = TestError {
        kind: ErrorKind::FlagDuplicate { original: Span { start: Position { index: 0 }, end: Position { index: 1 } } },
    };

    assert_eq!(error.description(), "duplicate flag");
}

