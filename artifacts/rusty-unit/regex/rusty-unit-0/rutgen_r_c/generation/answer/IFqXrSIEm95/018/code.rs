// Answer 0

#[test]
fn test_description_flag_dangling_negation() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct TestError {
        kind: ErrorKind,
    }

    impl TestError {
        fn description(&self) -> &str {
            use self::ErrorKind::*;
            match self.kind {
                // other cases omitted for brevity
                FlagDanglingNegation => "dangling flag negation operator",
                // default case omitted for brevity
                _ => unreachable!(),
            }
        }
    }

    let error = TestError {
        kind: ErrorKind::FlagDanglingNegation,
    };
    assert_eq!(error.description(), "dangling flag negation operator");
}

