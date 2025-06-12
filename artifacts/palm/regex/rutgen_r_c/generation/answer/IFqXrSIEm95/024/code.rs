// Answer 0

#[test]
fn test_error_description_decimal_invalid() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct MyError {
        kind: ErrorKind,
    }

    impl MyError {
        fn description(&self) -> &str {
            use self::ErrorKind::*;
            match self.kind {
                ErrorKind::DecimalInvalid => "invalid decimal literal",
                _ => "unknown error",
            }
        }
    }

    let error = MyError {
        kind: ErrorKind::DecimalInvalid,
    };

    assert_eq!(error.description(), "invalid decimal literal");
}

