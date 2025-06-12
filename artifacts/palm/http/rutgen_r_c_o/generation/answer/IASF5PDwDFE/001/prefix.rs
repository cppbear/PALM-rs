// Answer 0

#[test]
fn test_and_then_with_valid_parts() {
    let builder = Builder::new();
    let parts = Parts {
        method: Method::GET,
        uri: Uri::from_static("http://example.com"),
        version: Version::HTTP_1_1,
        headers: HeaderMap::new(),
        extensions: Extensions::new(),
        _priv: (),
    };
    
    let new_builder = builder.and_then(|_| Ok(parts));
}

#[test]
fn test_and_then_with_none_inner() {
    let builder = Builder::new();
    let parts = Parts {
        method: Method::GET,
        uri: Uri::from_static("http://example.com"),
        version: Version::HTTP_1_1,
        headers: HeaderMap::new(),
        extensions: Extensions::new(),
        _priv: (),
    };
    
    let new_builder = builder.and_then(|_| None);
}

#[test]
fn test_and_then_with_err_result() {
    let builder = Builder::new();
    let parts = Parts {
        method: Method::POST,
        uri: Uri::from_static("http://example.com"),
        version: Version::HTTP_1_0,
        headers: HeaderMap::new(),
        extensions: Extensions::new(),
        _priv: (),
    };
    
    let new_builder = builder.and_then(|_| Err(Error::new(ErrorKind::Other)));
}

#[test]
fn test_and_then_with_empty_headers() {
    let builder = Builder::new();
    let parts = Parts {
        method: Method::DELETE,
        uri: Uri::from_static("http://example.com"),
        version: Version::HTTP_2,
        headers: HeaderMap::new(),
        extensions: Extensions::new(),
        _priv: (),
    };
    
    let new_builder = builder.and_then(|_| Ok(parts));
}

#[test]
fn test_and_then_with_multiple_extensions() {
    let builder = Builder::new();
    let parts = Parts {
        method: Method::PUT,
        uri: Uri::from_static("http://example.com"),
        version: Version::HTTP_1_1,
        headers: HeaderMap::new(),
        extensions: {
            let mut ext = Extensions::new();
            ext.insert("key1".to_string(), "value1".to_string());
            ext.insert("key2".to_string(), "value2".to_string());
            ext
        },
        _priv: (),
    };
    
    let new_builder = builder.and_then(|_| Ok(parts));
}

#[test]
fn test_and_then_with_various_status_codes() {
    let builder = Builder::new();
    let status_codes = vec![
        StatusCode::OK,
        StatusCode::NOT_FOUND,
        StatusCode::INTERNAL_SERVER_ERROR,
    ];
    
    for status in status_codes {
        let parts = Parts {
            method: Method::PATCH,
            uri: Uri::from_static("http://example.com"),
            version: Version::HTTP_1_1,
            headers: HeaderMap::new(),
            extensions: Extensions::new(),
            _priv: (),
        };

        let new_builder = builder.and_then(|_| Ok(parts));
    }
}

