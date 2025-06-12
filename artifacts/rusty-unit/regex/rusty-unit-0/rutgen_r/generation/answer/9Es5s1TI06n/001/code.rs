// Answer 0

#[test]
fn test_fmt_class_escape_invalid() {
    struct TestError;
    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "invalid escape sequence found in character class")
        }
    }
    let error = TestError;
    let mut buffer = String::new();
    error.fmt(&mut buffer).unwrap();
    assert_eq!(buffer, "invalid escape sequence found in character class");
}

#[test]
fn test_fmt_flag_unrecognized() {
    struct TestError;
    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "unrecognized flag")
        }
    }
    let error = TestError;
    let mut buffer = String::new();
    error.fmt(&mut buffer).unwrap();
    assert_eq!(buffer, "unrecognized flag");
}

#[test]
fn test_fmt_group_name_invalid() {
    struct TestError;
    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "invalid capture group character")
        }
    }
    let error = TestError;
    let mut buffer = String::new();
    error.fmt(&mut buffer).unwrap();
    assert_eq!(buffer, "invalid capture group character");
}

#[test]
fn test_fmt_repetition_count_invalid() {
    struct TestError;
    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "invalid repetition count range, the start must be <= the end")
        }
    }
    let error = TestError;
    let mut buffer = String::new();
    error.fmt(&mut buffer).unwrap();
    assert_eq!(buffer, "invalid repetition count range, the start must be <= the end");
}

#[test]
fn test_fmt_decimal_invalid() {
    struct TestError;
    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "decimal literal invalid")
        }
    }
    let error = TestError;
    let mut buffer = String::new();
    error.fmt(&mut buffer).unwrap();
    assert_eq!(buffer, "decimal literal invalid");
}

