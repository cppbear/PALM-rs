// Answer 0

#[test]
fn test_fmt_class_escape_invalid() {
    struct ErrorKindClassEscapeInvalid;

    impl std::fmt::Display for ErrorKindClassEscapeInvalid {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "invalid escape sequence found in character class")
        }
    }

    let error = ErrorKindClassEscapeInvalid;

    let mut output = String::new();
    let result = error.fmt(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "invalid escape sequence found in character class");
}

#[test]
fn test_fmt_class_range_invalid() {
    struct ErrorKindClassRangeInvalid;

    impl std::fmt::Display for ErrorKindClassRangeInvalid {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "invalid character class range, the start must be <= the end")
        }
    }

    let error = ErrorKindClassRangeInvalid;

    let mut output = String::new();
    let result = error.fmt(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "invalid character class range, the start must be <= the end");
}

