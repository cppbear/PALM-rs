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
fn test_get_ref_status_code() {
    struct MyError;

    impl std::fmt::Debug for MyError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "MyError")
        }
    }

    impl std::error::Error for MyError {}

    let error = Error {
        inner: ErrorKind::StatusCode(Box::new(MyError)),
    };

    assert!(error.get_ref().is::<MyError>());
}

#[test]
fn test_get_ref_method() {
    struct MyError;

    impl std::fmt::Debug for MyError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "MyError")
        }
    }

    impl std::error::Error for MyError {}

    let error = Error {
        inner: ErrorKind::Method(Box::new(MyError)),
    };

    assert!(error.get_ref().is::<MyError>());
}

#[test]
fn test_get_ref_uri() {
    struct MyError;

    impl std::fmt::Debug for MyError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "MyError")
        }
    }

    impl std::error::Error for MyError {}

    let error = Error {
        inner: ErrorKind::Uri(Box::new(MyError)),
    };

    assert!(error.get_ref().is::<MyError>());
}

#[test]
fn test_get_ref_header_name() {
    struct MyError;

    impl std::fmt::Debug for MyError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "MyError")
        }
    }

    impl std::error::Error for MyError {}

    let error = Error {
        inner: ErrorKind::HeaderName(Box::new(MyError)),
    };

    assert!(error.get_ref().is::<MyError>());
} 

#[test]
fn test_get_ref_header_value() {
    struct MyError;

    impl std::fmt::Debug for MyError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "MyError")
        }
    }

    impl std::error::Error for MyError {}

    let error = Error {
        inner: ErrorKind::HeaderValue(Box::new(MyError)),
    };

    assert!(error.get_ref().is::<MyError>());
}

#[test]
fn test_get_ref_max_size_reached() {
    struct MyError;

    impl std::fmt::Debug for MyError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "MyError")
        }
    }

    impl std::error::Error for MyError {}

    let error = Error {
        inner: ErrorKind::MaxSizeReached(Box::new(MyError)),
    };

    assert!(error.get_ref().is::<MyError>());
} 

#[test]
fn test_get_ref_uri_parts() {
    struct MyError;

    impl std::fmt::Debug for MyError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "MyError")
        }
    }

    impl std::error::Error for MyError {}

    let error = Error {
        inner: ErrorKind::UriParts(Box::new(MyError)),
    };

    assert!(error.get_ref().is::<MyError>());
}

