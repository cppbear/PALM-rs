// Answer 0

#[test]
fn test_auxiliary_span_group_name_duplicate() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct TestError {
        kind: ErrorKind,
    }

    impl TestError {
        pub fn kind(&self) -> &ErrorKind {
            &self.kind
        }

        pub fn auxiliary_span(&self) -> Option<&Span> {
            use self::ErrorKind::*;
            match self.kind() {
                FlagDuplicate { ref original } => Some(original),
                FlagRepeatedNegation { ref original, .. } => Some(original),
                GroupNameDuplicate { ref original, .. } => Some(original),
                _ => None,
            }
        }
    }

    let original_span = Span { start: Position(0), end: Position(5) };
    let test_error = TestError {
        kind: ErrorKind::GroupNameDuplicate { original: original_span },
    };

    assert_eq!(test_error.auxiliary_span(), Some(&original_span));
}

#[test]
fn test_auxiliary_span_flag_duplicate() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct TestError {
        kind: ErrorKind,
    }

    impl TestError {
        pub fn kind(&self) -> &ErrorKind {
            &self.kind
        }

        pub fn auxiliary_span(&self) -> Option<&Span> {
            use self::ErrorKind::*;
            match self.kind() {
                FlagDuplicate { ref original } => Some(original),
                FlagRepeatedNegation { ref original, .. } => Some(original),
                GroupNameDuplicate { ref original, .. } => Some(original),
                _ => None,
            }
        }
    }

    let original_span = Span { start: Position(1), end: Position(4) };
    let test_error = TestError {
        kind: ErrorKind::FlagDuplicate { original: original_span },
    };

    assert_eq!(test_error.auxiliary_span(), Some(&original_span));
}

#[test]
fn test_auxiliary_span_flag_repeated_negation() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct TestError {
        kind: ErrorKind,
    }

    impl TestError {
        pub fn kind(&self) -> &ErrorKind {
            &self.kind
        }

        pub fn auxiliary_span(&self) -> Option<&Span> {
            use self::ErrorKind::*;
            match self.kind() {
                FlagDuplicate { ref original } => Some(original),
                FlagRepeatedNegation { ref original, .. } => Some(original),
                GroupNameDuplicate { ref original, .. } => Some(original),
                _ => None,
            }
        }
    }

    let original_span = Span { start: Position(2), end: Position(3) };
    let test_error = TestError {
        kind: ErrorKind::FlagRepeatedNegation { original: original_span },
    };

    assert_eq!(test_error.auxiliary_span(), Some(&original_span));
}

