// Answer 0

#[test]
fn test_fmt_decimal_invalid() {
    struct ErrorKind;

    impl fmt::Display for ErrorKind {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "decimal literal invalid")
        }
    }

    fn test_error_kind(fmt_error: ErrorKind) {
        let formatted = format!("{}", fmt_error);
        assert_eq!(formatted, "decimal literal invalid");
    }

    test_error_kind(ErrorKind);
}

#[test]
fn test_fmt_decimal_empty() {
    struct ErrorKind;

    impl fmt::Display for ErrorKind {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "decimal literal empty")
        }
    }

    fn test_error_kind(fmt_error: ErrorKind) {
        let formatted = format!("{}", fmt_error);
        assert_eq!(formatted, "decimal literal empty");
    }

    test_error_kind(ErrorKind);
}

