// Answer 0

#[derive(Debug)]
struct Error {
    inner: ErrorKind,
}

#[derive(Debug)]
enum ErrorKind {
    StatusCode(Box<dyn std::error::Error>),
    Method(Box<dyn std::error::Error>),
    Uri(Box<dyn std::error::Error>),
    UriParts(Box<dyn std::error::Error>),
    HeaderName(Box<dyn std::error::Error>),
    HeaderValue(Box<dyn std::error::Error>),
    MaxSizeReached(Box<dyn std::error::Error>),
}

impl Error {
    pub fn get_ref(&self) -> &(dyn std::error::Error + 'static) {
        use self::ErrorKind::*;

        match self.inner {
            StatusCode(ref e) => e,
            Method(ref e) => e,
            Uri(ref e) => e,
            UriParts(ref e) => e,
            HeaderName(ref e) => e,
            HeaderValue(ref e) => e,
            MaxSizeReached(ref e) => e,
        }
    }
}

#[test]
fn test_get_ref_header_name() {
    struct MockError;

    impl std::fmt::Debug for MockError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "MockError")
        }
    }

    impl std::error::Error for MockError {}

    let error = Error {
        inner: ErrorKind::HeaderName(Box::new(MockError)),
    };

    let inner_error: &dyn std::error::Error = error.get_ref();
    assert_eq!(format!("{:?}", inner_error), "MockError");
}

#[test]
fn test_get_ref_header_value() {
    struct MockError;

    impl std::fmt::Debug for MockError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "MockError")
        }
    }

    impl std::error::Error for MockError {}

    let error = Error {
        inner: ErrorKind::HeaderValue(Box::new(MockError)),
    };

    let inner_error: &dyn std::error::Error = error.get_ref();
    assert_eq!(format!("{:?}", inner_error), "MockError");
}

