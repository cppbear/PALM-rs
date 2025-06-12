// Answer 0

#[test]
fn test_description_repetition_count_unclosed() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct CustomError {
        kind: ErrorKind,
    }

    impl CustomError {
        fn description(&self) -> &str {
            use self::ErrorKind::*;
            match self.kind {
                ErrorKind::RepetitionCountUnclosed => "unclosed counted repetition",
                _ => unreachable!(),
            }
        }
    }

    let error = CustomError {
        kind: ErrorKind::RepetitionCountUnclosed,
    };
    assert_eq!(error.description(), "unclosed counted repetition");
}

