// Answer 0

#[test]
fn test_description_repetition_missing() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct TestError {
        kind: ErrorKind,
    }

    impl TestError {
        fn description(&self) -> &str {
            use self::ErrorKind::*;
            match self.kind {
                RepetitionMissing => "repetition operator missing expression",
                _ => "other error",
            }
        }
    }

    let error = TestError {
        kind: ErrorKind::RepetitionMissing,
    };

    assert_eq!(error.description(), "repetition operator missing expression");
}

