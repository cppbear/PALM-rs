// Answer 0

#[test]
fn test_from_str_valid_uri() {
    struct ValidUri;

    impl TryFrom<&str> for ValidUri {
        type Error = InvalidUri;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            if value.starts_with("http://") || value.starts_with("https://") {
                Ok(ValidUri)
            } else {
                Err(InvalidUri)
            }
        }
    }

    let uri_str = "http://example.com";
    let result = ValidUri::try_from(uri_str);
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

    let uri_str = "example.com";
    let result = InvalidUri::try_from(uri_str);
    assert!(result.is_err());
}

#[should_panic]
#[test]
fn test_from_str_should_panic_invalid_format() {
    struct PanicUri;

    impl TryFrom<&str> for PanicUri {
        type Error = &'static str;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            if value.is_empty() {
                panic!("Invalid format");
            }
            Ok(PanicUri)
        }
    }

    let empty_uri_str = "";
    let _result = PanicUri::try_from(empty_uri_str);
}

