// Answer 0

#[test]
fn test_from_str_valid_uri() {
    struct ValidUri;

    impl TryFrom<&str> for ValidUri {
        type Error = InvalidUri;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            if value.is_empty() {
                Err(InvalidUri)
            } else {
                Ok(ValidUri)
            }
        }
    }

    let result = from_str("http://example.com");
    assert!(result.is_ok());
}

#[test]
fn test_from_str_invalid_uri() {
    struct InvalidUri;

    impl TryFrom<&str> for InvalidUri {
        type Error = InvalidUri;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            Err(InvalidUri)
        }
    }

    let result = from_str("");
    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_from_str_panic() {
    struct PanicUri;

    impl TryFrom<&str> for PanicUri {
        type Error = InvalidUri;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            panic!("This should panic");

            // We will not reach this point
            Ok(PanicUri)
        }
    }

    let _ = from_str("http://example.com");
}

