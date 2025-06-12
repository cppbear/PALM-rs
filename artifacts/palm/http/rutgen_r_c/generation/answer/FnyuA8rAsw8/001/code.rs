// Answer 0

#[test]
fn test_headers_mut_success() {
    use crate::response::Builder;
    use crate::header::{HeaderMap, HeaderValue};
    use crate::status::StatusCode;
    use crate::version::Version;
    use crate::{Extensions, Result};

    // Helper struct to simulate Parts creation
    #[derive(Debug)]
    struct MockParts {
        headers: HeaderMap<HeaderValue>,
    }

    impl MockParts {
        fn new() -> Self {
            let headers = HeaderMap::default(); // Assuming a default implementation exists
            Self { headers }
        }
    }

    let mut builder = Builder {
        inner: Ok(MockParts::new()), // Simulating a successful state
    };

    // Mutating headers
    let headers = builder.headers_mut().unwrap();
    headers.insert("Test-Header", HeaderValue::from_static("TestValue"));

    // Retrieving the headers back
    let headers_ref = builder.headers_ref().unwrap();
    assert_eq!(headers_ref.get("Test-Header").unwrap(), "TestValue");
}

#[test]
fn test_headers_mut_error() {
    use crate::response::Builder;
    use crate::header::{HeaderMap, HeaderValue};

    // Helper struct to simulate an error state
    #[derive(Debug)]
    struct MockParts;

    let mut builder = Builder {
        inner: Err(crate::Error), // Simulating an error state
    };

    // Expecting None when there is an error
    assert!(builder.headers_mut().is_none());
} 

#[test]
fn test_headers_mut_multiple_inserts() {
    use crate::response::Builder;
    use crate::header::{HeaderMap, HeaderValue};
    use crate::status::StatusCode;
    use crate::version::Version;
    use crate::{Extensions, Result};

    #[derive(Debug)]
    struct MockParts {
        headers: HeaderMap<HeaderValue>,
    }

    impl MockParts {
        fn new() -> Self {
            let headers = HeaderMap::default();
            Self { headers }
        }
    }

    let mut builder = Builder {
        inner: Ok(MockParts::new()),
    };

    {
        let headers = builder.headers_mut().unwrap();
        headers.insert("Header-1", HeaderValue::from_static("Value1"));
        headers.insert("Header-2", HeaderValue::from_static("Value2"));
    }

    let headers_ref = builder.headers_ref().unwrap();
    assert_eq!(headers_ref.get("Header-1").unwrap(), "Value1");
    assert_eq!(headers_ref.get("Header-2").unwrap(), "Value2");
}  

#[test]
fn test_headers_mut_empty_state() {
    use crate::response::Builder;
    use crate::header::{HeaderMap, HeaderValue};

    #[derive(Debug)]
    struct MockParts {
        headers: HeaderMap<HeaderValue>,
    }

    impl MockParts {
        fn new() -> Self {
            let headers = HeaderMap::default();
            Self { headers }
        }
    }

    let mut builder = Builder {
        inner: Ok(MockParts::new()),
    };

    let headers = builder.headers_mut().unwrap();
    assert!(headers.is_empty());
}  

