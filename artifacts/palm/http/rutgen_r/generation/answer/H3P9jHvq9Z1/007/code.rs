// Answer 0

#[derive(Debug)]
struct Error {
    inner: ErrorKind,
}

#[derive(Debug)]
enum ErrorKind {
    StatusCode(Box<dyn std::error::Error + 'static>),
    Method(Box<dyn std::error::Error + 'static>),
    Uri(Box<dyn std::error::Error + 'static>),
    UriParts(Box<dyn std::error::Error + 'static>),
    HeaderName(Box<dyn std::error::Error + 'static>),
    HeaderValue(Box<dyn std::error::Error + 'static>),
    MaxSizeReached(Box<dyn std::error::Error + 'static>),
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

#[derive(Debug)]
struct CustomError;

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Custom error")
    }
}

impl std::error::Error for CustomError {}

#[test]
fn test_get_ref_status_code() {
    let e: Box<dyn std::error::Error + 'static> = Box::new(CustomError);
    let error = Error {
        inner: ErrorKind::StatusCode(e),
    };
    let ref_error = error.get_ref();
    assert_eq!(ref_error.to_string(), "Custom error");
}

#[test]
fn test_get_ref_method() {
    let e: Box<dyn std::error::Error + 'static> = Box::new(CustomError);
    let error = Error {
        inner: ErrorKind::Method(e),
    };
    let ref_error = error.get_ref();
    assert_eq!(ref_error.to_string(), "Custom error");
}

#[test]
fn test_get_ref_uri() {
    let e: Box<dyn std::error::Error + 'static> = Box::new(CustomError);
    let error = Error {
        inner: ErrorKind::Uri(e),
    };
    let ref_error = error.get_ref();
    assert_eq!(ref_error.to_string(), "Custom error");
}

#[test]
fn test_get_ref_uri_parts() {
    let e: Box<dyn std::error::Error + 'static> = Box::new(CustomError);
    let error = Error {
        inner: ErrorKind::UriParts(e),
    };
    let ref_error = error.get_ref();
    assert_eq!(ref_error.to_string(), "Custom error");
}

#[test]
fn test_get_ref_header_name() {
    let e: Box<dyn std::error::Error + 'static> = Box::new(CustomError);
    let error = Error {
        inner: ErrorKind::HeaderName(e),
    };
    let ref_error = error.get_ref();
    assert_eq!(ref_error.to_string(), "Custom error");
}

#[test]
fn test_get_ref_header_value() {
    let e: Box<dyn std::error::Error + 'static> = Box::new(CustomError);
    let error = Error {
        inner: ErrorKind::HeaderValue(e),
    };
    let ref_error = error.get_ref();
    assert_eq!(ref_error.to_string(), "Custom error");
}

#[test]
fn test_get_ref_max_size_reached() {
    let e: Box<dyn std::error::Error + 'static> = Box::new(CustomError);
    let error = Error {
        inner: ErrorKind::MaxSizeReached(e),
    };
    let ref_error = error.get_ref();
    assert_eq!(ref_error.to_string(), "Custom error");
}

