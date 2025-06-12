// Answer 0

#[test]
fn test_response_fmt_valid_input() {
    let status_code = StatusCode(1.try_into().unwrap());
    let version = Version(Http);
    let headers = HeaderMap {
        mask: Size::default(),
        indices: Box::new([]),
        entries: vec![Bucket::default()],
        extra_values: vec![],
        danger: Danger::default(),
    };
    let body = vec![0; 4096]; // max body size
    let parts = Parts {
        status: status_code,
        version: version,
        headers: headers,
        extensions: Extensions::default(),
        _priv: (),
    };
    
    let response = Response::from_parts(parts, body);
    let _ = format!("{:?}", response);
}

#[test]
fn test_response_fmt_minimum_body_size() {
    let status_code = StatusCode(200.try_into().unwrap());
    let version = Version(Http);
    let headers = HeaderMap {
        mask: Size::default(),
        indices: Box::new([]),
        entries: vec![Bucket::default()],
        extra_values: vec![],
        danger: Danger::default(),
    };
    let body = vec![]; // empty body
    let parts = Parts {
        status: status_code,
        version: version,
        headers: headers,
        extensions: Extensions::default(),
        _priv: (),
    };
    
    let response = Response::from_parts(parts, body);
    let _ = format!("{:?}", response);
}

#[test]
fn test_response_fmt_full_capacity_body() {
    let status_code = StatusCode(65535.try_into().unwrap());
    let version = Version(Http);
    let headers = HeaderMap {
        mask: Size::default(),
        indices: Box::new([]),
        entries: vec![Bucket::default(); 1024], // maximum capacity
        extra_values: vec![],
        danger: Danger::default(),
    };
    let body = vec![1; 4096]; // body at max size
    let parts = Parts {
        status: status_code,
        version: version,
        headers: headers,
        extensions: Extensions::default(),
        _priv: (),
    };

    let response = Response::from_parts(parts, body);
    let _ = format!("{:?}", response);
}

#[test]
#[should_panic]
fn test_response_fmt_invalid_status_code() {
    let status_code = StatusCode(0.try_into().unwrap()); // invalid status code
    let version = Version(Http);
    let headers = HeaderMap {
        mask: Size::default(),
        indices: Box::new([]),
        entries: vec![Bucket::default()],
        extra_values: vec![],
        danger: Danger::default(),
    };
    let body = vec![0; 100]; // valid body
    let parts = Parts {
        status: status_code,
        version: version,
        headers: headers,
        extensions: Extensions::default(),
        _priv: (),
    };

    let response = Response::from_parts(parts, body);
    let _ = format!("{:?}", response);
}

