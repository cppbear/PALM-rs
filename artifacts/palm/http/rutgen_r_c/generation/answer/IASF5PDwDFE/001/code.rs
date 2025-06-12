// Answer 0

#[test]
fn test_and_then_success() {
    use crate::header::{HeaderMap, HeaderName, HeaderValue};
    use crate::status::StatusCode;
    use crate::version::Version;
    use crate::{Extensions, Result};

    struct MockParts {
        status: StatusCode,
        version: Version,
        headers: HeaderMap<HeaderValue>,
        extensions: Extensions,
        _priv: (),
    }

    let initial_builder = Builder {
        inner: Ok(MockParts {
            status: StatusCode::OK,
            version: Version::HTTP_11,
            headers: HeaderMap::new(),
            extensions: Extensions::new(),
            _priv: (),
        }),
    };

    let new_builder = initial_builder.and_then(|parts| {
        Ok(MockParts {
            status: StatusCode::NOT_FOUND,
            version: parts.version,
            headers: parts.headers,
            extensions: parts.extensions,
            _priv: (),
        })
    });

    assert!(new_builder.inner.is_ok());
}

#[test]
#[should_panic]
fn test_and_then_panic_on_error() {
    use crate::header::{HeaderMap, HeaderValue};
    use crate::status::StatusCode;
    use crate::version::Version;
    use crate::{Extensions, Result};

    struct MockParts {
        status: StatusCode,
        version: Version,
        headers: HeaderMap<HeaderValue>,
        extensions: Extensions,
        _priv: (),
    }

    let initial_builder = Builder {
        inner: Err(Error { inner: ErrorKind::Other }),
    };

    let _ = initial_builder.and_then(|_| {
        panic!("This should trigger a panic due to the initial error.");
    });
}

#[test]
fn test_and_then_no_parts() {
    use crate::header::{HeaderMap, HeaderValue};
    use crate::status::StatusCode;
    use crate::version::Version;
    use crate::{Extensions, Result};

    struct MockParts {
        status: StatusCode,
        version: Version,
        headers: HeaderMap<HeaderValue>,
        extensions: Extensions,
        _priv: (),
    }

    let initial_builder = Builder {
        inner: Ok(MockParts {
            status: StatusCode::OK,
            version: Version::HTTP_11,
            headers: HeaderMap::new(),
            extensions: Extensions::new(),
            _priv: (),
        }),
    };

    let new_builder = initial_builder.and_then(|parts| {
        Ok(MockParts {
            status: parts.status,
            version: Version::HTTP_10,
            headers: parts.headers,
            extensions: parts.extensions,
            _priv: (),
        })
    });

    assert!(new_builder.inner.is_ok());
    assert_eq!(new_builder.inner.unwrap().version, Version::HTTP_10);
}

