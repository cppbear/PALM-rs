// Answer 0

#[test]
fn test_fmt_nest_limit_exceeded() {
    struct Span;

    impl fmt::Display for ErrorKind {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            use self::ErrorKind::*;
            match *self {
                NestLimitExceeded(limit) => {
                    write!(f, "exceed the maximum number of nested parentheses/brackets ({})", limit)
                }
                _ => write!(f, "not a valid error message"),
            }
        }
    }

    let error = ErrorKind::NestLimitExceeded(42);
    let mut buffer = String::new();
    let result = error.fmt(&mut buffer);

    assert!(result.is_ok());
    assert_eq!(buffer, "exceed the maximum number of nested parentheses/brackets (42)");
}

#[test]
fn test_fmt_nest_limit_exceeded_zero() {
    struct Span;

    impl fmt::Display for ErrorKind {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            use self::ErrorKind::*;
            match *self {
                NestLimitExceeded(limit) => {
                    write!(f, "exceed the maximum number of nested parentheses/brackets ({})", limit)
                }
                _ => write!(f, "not a valid error message"),
            }
        }
    }

    let error = ErrorKind::NestLimitExceeded(0);
    let mut buffer = String::new();
    let result = error.fmt(&mut buffer);

    assert!(result.is_ok());
    assert_eq!(buffer, "exceed the maximum number of nested parentheses/brackets (0)");
}

