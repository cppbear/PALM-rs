// Answer 0

#[test]
fn test_new_parts_default() {
    let parts = Parts::new();
}

#[test]
fn test_new_parts_with_status_code() {
    let status_code = StatusCode(100.try_into().unwrap());
    let parts = Parts {
        status: status_code,
        version: Version::default(),
        headers: HeaderMap::default(),
        extensions: Extensions::default(),
        _priv: (),
    };
}

#[test]
fn test_new_parts_with_http_version() {
    let version = Version(Http);
    let parts = Parts {
        status: StatusCode::default(),
        version,
        headers: HeaderMap::default(),
        extensions: Extensions::default(),
        _priv: (),
    };
}

#[test]
fn test_new_parts_with_custom_headers() {
    let headers = HeaderMap::default();
    let parts = Parts {
        status: StatusCode::default(),
        version: Version::default(),
        headers,
        extensions: Extensions::default(),
        _priv: (),
    };
}

#[test]
fn test_new_parts_with_extensions() {
    let extensions = Extensions {
        map: Some(Box::<AnyMap>::default()),
    };
    let parts = Parts {
        status: StatusCode::default(),
        version: Version::default(),
        headers: HeaderMap::default(),
        extensions,
        _priv: (),
    };
}

