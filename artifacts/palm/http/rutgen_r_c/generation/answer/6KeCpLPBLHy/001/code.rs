// Answer 0

#[test]
fn test_headers_ref_ok() {
    let mut header_map = HeaderMap::default();
    let header_name = HeaderName::from_bytes(b"Accept").unwrap();
    let header_value = HeaderValue::from_bytes(b"text/html").unwrap();
    header_map.append(header_name.clone(), header_value.clone());

    let header_name_custom = HeaderName::from_bytes(b"X-Custom-Foo").unwrap();
    let header_value_custom = HeaderValue::from_bytes(b"bar").unwrap();
    header_map.append(header_name_custom.clone(), header_value_custom.clone());

    let parts = Parts {
        headers: header_map.clone(),
        ..Parts::default()
    };

    let builder = Builder {
        inner: Ok(parts),
    };

    let headers = builder.headers_ref().unwrap();
    assert_eq!(headers[&header_name], header_value);
    assert_eq!(headers[&header_name_custom], header_value_custom);
}

#[test]
fn test_headers_ref_err() {
    let builder = Builder {
        inner: Err(ErrorKind::SomeError.into()),
    };

    let headers = builder.headers_ref();
    assert!(headers.is_none());
}

#[test]
fn test_headers_ref_empty() {
    let parts = Parts {
        headers: HeaderMap::default(),
        ..Parts::default()
    };

    let builder = Builder {
        inner: Ok(parts),
    };

    let headers = builder.headers_ref().unwrap();
    assert!(headers.is_empty());
}

