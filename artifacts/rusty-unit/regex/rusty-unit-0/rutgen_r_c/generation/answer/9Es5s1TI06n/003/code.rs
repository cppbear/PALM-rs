// Answer 0

#[test]
fn test_fmt_unsupported_backreference() {
    struct TestError {
        kind: ErrorKind,
    }

    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.kind.fmt(f)
        }
    }

    let error = TestError {
        kind: ErrorKind::UnsupportedBackreference,
    };
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "backreferences are not supported");
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

    let error = TestError {
        kind: ErrorKind::FlagDuplicate { original: Span { start: Position::from(0), end: Position::from(0) } },
    };
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "duplicate flag");
}

