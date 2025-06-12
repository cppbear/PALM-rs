// Answer 0

#[test]
fn test_fmt_flag_repeated_negation() {
    struct TestError {
        kind: ErrorKind,
    }

    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.kind.fmt(f)
        }
    }

    let original_span = Span {
        start: Position { /* Initialize position struct here */ },
        end: Position { /* Initialize position struct here */ },
    };

    let error_variant = ErrorKind::FlagRepeatedNegation { original: original_span };
    let mut buf = String::new();
    let result = write!(&mut buf, "{}", TestError { kind: error_variant });

    assert!(result.is_ok());
    assert_eq!(buf, "flag negation operator repeated");
}

#[test]
fn test_fmt_flag_duplicate() {
    struct TestError {
        kind: ErrorKind,
    }

    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.kind.fmt(f)
        }
    }

    let error_variant = ErrorKind::FlagDuplicate { original: Span { /* same as above */ } };
    let mut buf = String::new();
    let result = write!(&mut buf, "{}", TestError { kind: error_variant });

    assert!(result.is_ok());
    assert_eq!(buf, "duplicate flag");
}

