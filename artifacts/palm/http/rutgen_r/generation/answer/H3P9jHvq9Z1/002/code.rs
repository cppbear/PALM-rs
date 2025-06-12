// Answer 0

#[derive(Debug)]
struct HeaderValueError;

impl std::fmt::Display for HeaderValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "HeaderValue error")
    }
}

impl std::error::Error for HeaderValueError {}

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

#[derive(Debug)]
struct Error {
    inner: ErrorKind,
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
fn test_get_ref_header_value() {
    let error = Error {
        inner: ErrorKind::HeaderValue(Box::new(HeaderValueError)),
    };

    let inner_error: &dyn std::error::Error = error.get_ref();
    assert_eq!(inner_error.to_string(), "HeaderValue error");
}

#[test]
fn test_get_ref_status_code() {
    let error = Error {
        inner: ErrorKind::StatusCode(Box::new(HeaderValueError)),
    };

    let inner_error: &dyn std::error::Error = error.get_ref();
    assert_eq!(inner_error.to_string(), "HeaderValue error");
}

#[test]
fn test_get_ref_method() {
    let error = Error {
        inner: ErrorKind::Method(Box::new(HeaderValueError)),
    };

    let inner_error: &dyn std::error::Error = error.get_ref();
    assert_eq!(inner_error.to_string(), "HeaderValue error");
}

#[test]
fn test_get_ref_uri() {
    let error = Error {
        inner: ErrorKind::Uri(Box::new(HeaderValueError)),
    };

    let inner_error: &dyn std::error::Error = error.get_ref();
    assert_eq!(inner_error.to_string(), "HeaderValue error");
}

#[test]
fn test_get_ref_uri_parts() {
    let error = Error {
        inner: ErrorKind::UriParts(Box::new(HeaderValueError)),
    };

    let inner_error: &dyn std::error::Error = error.get_ref();
    assert_eq!(inner_error.to_string(), "HeaderValue error");
}

#[test]
fn test_get_ref_header_name() {
    let error = Error {
        inner: ErrorKind::HeaderName(Box::new(HeaderValueError)),
    };

    let inner_error: &dyn std::error::Error = error.get_ref();
    assert_eq!(inner_error.to_string(), "HeaderValue error");
}

#[test]
fn test_get_ref_max_size_reached() {
    let error = Error {
        inner: ErrorKind::MaxSizeReached(Box::new(HeaderValueError)),
    };

    let inner_error: &dyn std::error::Error = error.get_ref();
    assert_eq!(inner_error.to_string(), "HeaderValue error");
}

