// Answer 0

#[test]
fn test_from_parts_valid_response() {
    let parts = Parts {
        status: StatusCode::OK,
        version: Version::HTTP_11,
        headers: HeaderMap::new(),
        extensions: Extensions::default(),
        _priv: (),
    };
    let body = "valid body";
    let response = Response::from_parts(parts, body);
}

#[test]
fn test_from_parts_with_empty_body() {
    let parts = Parts {
        status: StatusCode::BAD_REQUEST,
        version: Version::HTTP_10,
        headers: HeaderMap::new(),
        extensions: Extensions::default(),
        _priv: (),
    };
    let body: &str = "";
    let response = Response::from_parts(parts, body);
}

#[test]
fn test_from_parts_with_valid_header() {
    let mut headers = HeaderMap::new();
    headers.insert(HeaderName::from_static("content-type"), HeaderValue::from_static("text/plain"));
    
    let parts = Parts {
        status: StatusCode::CREATED,
        version: Version::HTTP_11,
        headers,
        extensions: Extensions::default(),
        _priv: (),
    };
    let body = "body with valid header";
    let response = Response::from_parts(parts, body);
}

#[test]
fn test_from_parts_with_empty_extensions() {
    let parts = Parts {
        status: StatusCode::NO_CONTENT,
        version: Version::HTTP_20,
        headers: HeaderMap::new(),
        extensions: Extensions::default(),
        _priv: (),
    };
    let body = "body with no extensions";
    let response = Response::from_parts(parts, body);
}

#[test]
#[should_panic]
fn test_from_parts_exceeding_body_length() {
    let parts = Parts {
        status: StatusCode::INTERNAL_SERVER_ERROR,
        version: Version::HTTP_11,
        headers: HeaderMap::new(),
        extensions: Extensions::default(),
        _priv: (),
    };
    let body = "This is a test body that exceeds the maximum allowable length of 4096 characters...";
    let response = Response::from_parts(parts, body);
}

#[test]
fn test_from_parts_status_range() {
    let parts_ok = Parts {
        status: StatusCode::OK,
        version: Version::HTTP_11,
        headers: HeaderMap::new(),
        extensions: Extensions::default(),
        _priv: (),
    };
    let body_ok = "body for status OK";
    let response_ok = Response::from_parts(parts_ok, body_ok);
    
    let parts_server_error = Parts {
        status: StatusCode::INTERNAL_SERVER_ERROR,
        version: Version::HTTP_20,
        headers: HeaderMap::new(),
        extensions: Extensions::default(),
        _priv: (),
    };
    let body_server_error = "body for status 500";
    let response_server_error = Response::from_parts(parts_server_error, body_server_error);
}

