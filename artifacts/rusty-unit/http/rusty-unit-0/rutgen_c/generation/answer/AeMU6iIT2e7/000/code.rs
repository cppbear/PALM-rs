// Answer 0

#[test]
fn test_error_fmt_display_invalid_uri_char() {
    struct InvalidUriChar;
    impl fmt::Display for InvalidUriChar {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Invalid URI character")
        }
    }
    
    let error = Error { inner: ErrorKind::Uri(uri::InvalidUri) };
    let mut output = String::new();
    let result = {
        let mut formatter = fmt::Formatter::new();
        error.fmt(&mut formatter)
    };
    
    assert!(result.is_ok());
}

#[test]
fn test_error_fmt_display_empty_error() {
    struct EmptyError;
    impl fmt::Display for EmptyError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Empty error")
        }
    }

    let error = Error { inner: ErrorKind::Empty };
    let mut output = String::new();
    let result = {
        let mut formatter = fmt::Formatter::new();
        error.fmt(&mut formatter)
    };

    assert!(result.is_ok());
}

#[test]
fn test_error_fmt_display_scheme_missing() {
    struct SchemeMissingError;
    impl fmt::Display for SchemeMissingError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Scheme is missing")
        }
    }

    let error = Error { inner: ErrorKind::SchemeMissing };
    let mut output = String::new();
    let result = {
        let mut formatter = fmt::Formatter::new();
        error.fmt(&mut formatter)
    };

    assert!(result.is_ok());
}

