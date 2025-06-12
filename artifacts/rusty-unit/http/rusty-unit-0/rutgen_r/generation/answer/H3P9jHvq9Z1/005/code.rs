// Answer 0

#[derive(Debug)]
struct InnerError {
    kind: ErrorKind,
}

#[derive(Debug)]
enum ErrorKind {
    StatusCode(std::io::Error),
    Method(std::io::Error),
    Uri(std::io::Error),
    UriParts(std::io::Error),
    HeaderName(std::io::Error),
    HeaderValue(std::io::Error),
    MaxSizeReached(std::io::Error),
}

#[derive(Debug)]
struct Error {
    inner: InnerError,
}

impl Error {
    pub fn get_ref(&self) -> &(dyn std::error::Error + 'static) {
        use ErrorKind::*;

        match self.inner.kind {
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
fn test_get_ref_uri_error() {
    let io_error = std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid uri");
    let error = Error {
        inner: InnerError {
            kind: ErrorKind::Uri(io_error),
        },
    };

    let returned_error: &dyn std::error::Error = error.get_ref();
    assert_eq!(returned_error.to_string(), "invalid uri");
}

#[test]
fn test_get_ref_status_code_error() {
    let io_error = std::io::Error::new(std::io::ErrorKind::Other, "status code error");
    let error = Error {
        inner: InnerError {
            kind: ErrorKind::StatusCode(io_error),
        },
    };

    let returned_error: &dyn std::error::Error = error.get_ref();
    assert_eq!(returned_error.to_string(), "status code error");
}

#[test]
fn test_get_ref_method_error() {
    let io_error = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "method error");
    let error = Error {
        inner: InnerError {
            kind: ErrorKind::Method(io_error),
        },
    };

    let returned_error: &dyn std::error::Error = error.get_ref();
    assert_eq!(returned_error.to_string(), "method error");
}

#[test]
fn test_get_ref_uri_parts_error() {
    let io_error = std::io::Error::new(std::io::ErrorKind::InvalidData, "uri parts error");
    let error = Error {
        inner: InnerError {
            kind: ErrorKind::UriParts(io_error),
        },
    };

    let returned_error: &dyn std::error::Error = error.get_ref();
    assert_eq!(returned_error.to_string(), "uri parts error");
}

#[test]
fn test_get_ref_header_name_error() {
    let io_error = std::io::Error::new(std::io::ErrorKind::InvalidInput, "header name error");
    let error = Error {
        inner: InnerError {
            kind: ErrorKind::HeaderName(io_error),
        },
    };

    let returned_error: &dyn std::error::Error = error.get_ref();
    assert_eq!(returned_error.to_string(), "header name error");
}

#[test]
fn test_get_ref_header_value_error() {
    let io_error = std::io::Error::new(std::io::ErrorKind::InvalidInput, "header value error");
    let error = Error {
        inner: InnerError {
            kind: ErrorKind::HeaderValue(io_error),
        },
    };

    let returned_error: &dyn std::error::Error = error.get_ref();
    assert_eq!(returned_error.to_string(), "header value error");
}

#[test]
fn test_get_ref_max_size_reached_error() {
    let io_error = std::io::Error::new(std::io::ErrorKind::Other, "max size reached");
    let error = Error {
        inner: InnerError {
            kind: ErrorKind::MaxSizeReached(io_error),
        },
    };

    let returned_error: &dyn std::error::Error = error.get_ref();
    assert_eq!(returned_error.to_string(), "max size reached");
}

