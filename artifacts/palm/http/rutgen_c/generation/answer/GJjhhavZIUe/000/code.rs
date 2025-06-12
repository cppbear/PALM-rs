// Answer 0

#[test]
fn test_fmt_debug_tuple() {
    struct InvalidStatusCode;
    struct InvalidMethod;
    struct InvalidUri;
    struct InvalidUriParts;
    struct InvalidHeaderName;
    struct InvalidHeaderValue;
    struct MaxSizeReached;

    impl fmt::Debug for InvalidStatusCode {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("InvalidStatusCode")
        }
    }

    impl fmt::Debug for InvalidMethod {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("InvalidMethod")
        }
    }

    impl fmt::Debug for InvalidUri {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("InvalidUri")
        }
    }

    impl fmt::Debug for InvalidUriParts {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("InvalidUriParts")
        }
    }

    impl fmt::Debug for InvalidHeaderName {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("InvalidHeaderName")
        }
    }

    impl fmt::Debug for InvalidHeaderValue {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("InvalidHeaderValue")
        }
    }

    impl fmt::Debug for MaxSizeReached {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("MaxSizeReached")
        }
    }

    let error_status_code = Error {
        inner: ErrorKind::StatusCode(InvalidStatusCode),
    };
    let error_method = Error {
        inner: ErrorKind::Method(InvalidMethod),
    };

    let mut buffer = String::new();
    let result = write!(&mut buffer, "{:?}", error_status_code);
    assert!(result.is_ok());
    assert_eq!(buffer, "http::Error(InvalidStatusCode)");

    buffer.clear();
    let result = write!(&mut buffer, "{:?}", error_method);
    assert!(result.is_ok());
    assert_eq!(buffer, "http::Error(InvalidMethod)");
}

