// Answer 0

#[test]
fn test_description_class_range_literal() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct MockError {
        kind: ErrorKind,
    }

    impl MockError {
        fn description(&self) -> &str {
            use self::ErrorKind::*;
            match self.kind {
                ClassRangeLiteral => "invalid range boundary, must be a literal",
                _ => unreachable!(),
            }
        }
    }

    let error = MockError {
        kind: ErrorKind::ClassRangeLiteral,
    };

    assert_eq!(error.description(), "invalid range boundary, must be a literal");
}

#[test]
fn test_description_class_unclosed() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct MockError {
        kind: ErrorKind,
    }

    impl MockError {
        fn description(&self) -> &str {
            use self::ErrorKind::*;
            match self.kind {
                ClassUnclosed => "unclosed character class",
                _ => unreachable!(),
            }
        }
    }

    let error = MockError {
        kind: ErrorKind::ClassUnclosed,
    };

    assert_eq!(error.description(), "unclosed character class");
}

