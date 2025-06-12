// Answer 0

#[test]
fn test_headers_ref_with_valid_headers() {
    let mut builder = Builder::new();
    let header_map = HeaderMap::default();
    builder = builder.header("Accept", "text/html");
    builder = builder.header("X-Custom-Foo", "bar");
    let res = builder.and_then(|parts| {
        Ok(Parts {
            headers: header_map,
            ..parts
        })
    });
    let headers = res.headers_ref().unwrap();
}

#[test]
fn test_headers_ref_with_no_headers() {
    let mut builder = Builder::new();
    let res = builder.and_then(|parts| {
        Ok(Parts {
            headers: HeaderMap::default(),
            ..parts
        })
    });
    let headers = res.headers_ref();
    assert!(headers.is_none());
}

#[test]
fn test_headers_ref_with_large_header_value() {
    let mut builder = Builder::new();
    let long_value = "x".repeat(1024);
    builder = builder.header("Large-Header", long_value);
    let header_map = HeaderMap::default();
    let res = builder.and_then(|parts| {
        Ok(Parts {
            headers: header_map,
            ..parts
        })
    });
    let headers = res.headers_ref().unwrap();
}

#[test]
fn test_headers_ref_with_multiple_headers() {
    let mut builder = Builder::new();
    for i in 1..=5 {
        let key = format!("Header-{}", i);
        let value = format!("Value-{}", i);
        builder = builder.header(key, value);
    }
    let header_map = HeaderMap::default();
    let res = builder.and_then(|parts| {
        Ok(Parts {
            headers: header_map,
            ..parts
        })
    });
    let headers = res.headers_ref().unwrap();
}

#[test]
fn test_headers_ref_with_large_number_of_headers() {
    let mut builder = Builder::new();
    let header_map = HeaderMap::default();
    for i in 1..=100 {
        let key = format!("Header-{}", i);
        let value = format!("Value-{}", i);
        builder = builder.header(key, value);
    }
    let res = builder.and_then(|parts| {
        Ok(Parts {
            headers: header_map,
            ..parts
        })
    });
    let headers = res.headers_ref().unwrap();
}

