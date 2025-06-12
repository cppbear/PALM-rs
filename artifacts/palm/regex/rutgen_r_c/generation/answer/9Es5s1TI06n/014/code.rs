// Answer 0

#[test]
fn test_flag_unrecognized_display() {
    struct TestError {
        kind: ErrorKind,
    }
    
    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.kind)
        }
    }
    
    let error = TestError {
        kind: ErrorKind::FlagUnrecognized,
    };

    let mut output = String::new();
    {
        let mut formatter = fmt::Formatter::new(&mut output);
        error.fmt(&mut formatter).unwrap();
    }
    
    assert_eq!(output, "unrecognized flag");
}

#[test]
fn test_flag_duplicate_display() {
    struct TestError {
        kind: ErrorKind,
    }

    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.kind)
        }
    }

    let error = TestError {
        kind: ErrorKind::FlagDuplicate { original: Span { start: 0, end: 0 } },
    };

    let mut output = String::new();
    {
        let mut formatter = fmt::Formatter::new(&mut output);
        error.fmt(&mut formatter).unwrap();
    }

    assert_eq!(output, "duplicate flag");
}

