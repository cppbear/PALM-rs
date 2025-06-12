// Answer 0

#[test]
fn test_status_with_ok_status_code() {
    use std::num::NonZeroU16;
    use http::{Response, StatusCode, Parts};

    let status_code = StatusCode(NonZeroU16::new(200).unwrap());
    let parts = Parts { status: status_code, version: Version::default(), headers: HeaderMap::new(), extensions: Extensions::default(), _priv: () };
    let response = Response::from_parts(parts, ());
    
    assert_eq!(response.status(), status_code);
}

#[test]
fn test_status_with_not_found_status_code() {
    use std::num::NonZeroU16;
    use http::{Response, StatusCode, Parts};

    let status_code = StatusCode(NonZeroU16::new(404).unwrap());
    let parts = Parts { status: status_code, version: Version::default(), headers: HeaderMap::new(), extensions: Extensions::default(), _priv: () };
    let response = Response::from_parts(parts, ());
    
    assert_eq!(response.status(), status_code);
}

#[test]
fn test_status_with_internal_server_error_code() {
    use std::num::NonZeroU16;
    use http::{Response, StatusCode, Parts};

    let status_code = StatusCode(NonZeroU16::new(500).unwrap());
    let parts = Parts { status: status_code, version: Version::default(), headers: HeaderMap::new(), extensions: Extensions::default(), _priv: () };
    let response = Response::from_parts(parts, ());
    
    assert_eq!(response.status(), status_code);
}

#[test]
fn test_status_with_custom_status_code() {
    use std::num::NonZeroU16;
    use http::{Response, StatusCode, Parts};

    let status_code = StatusCode(NonZeroU16::new(418).unwrap()); // I'm a teapot
    let parts = Parts { status: status_code, version: Version::default(), headers: HeaderMap::new(), extensions: Extensions::default(), _priv: () };
    let response = Response::from_parts(parts, ());
    
    assert_eq!(response.status(), status_code);
}

