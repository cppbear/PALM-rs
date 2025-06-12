// Answer 0

#[test]
fn test_from_name_valid_header_name() {
    use http::header::{HeaderValue, HeaderName};

    let valid_header_name = HeaderName::from_bytes(b"accept").unwrap();
    let val = HeaderValue::from_name(valid_header_name);
    assert_eq!(val, HeaderValue::from_bytes(b"accept").unwrap());
}

#[test]
fn test_from_name_another_valid_header_name() {
    use http::header::{HeaderValue, HeaderName};

    let valid_header_name = HeaderName::from_bytes(b"content-type").unwrap();
    let val = HeaderValue::from_name(valid_header_name);
    assert_eq!(val, HeaderValue::from_bytes(b"content-type").unwrap());
}

#[test]
#[should_panic]
fn test_from_name_invalid_header_name() {
    use http::header::{HeaderName};

    let invalid_header_name = HeaderName::from_bytes(b"*invalid*").unwrap_err();
    let _val = HeaderValue::from_name(invalid_header_name);
}

#[test]
fn test_from_name_empty_header_name() {
    use http::header::{HeaderValue, HeaderName};

    let empty_header_name = HeaderName::from_bytes(b"").unwrap();
    let val = HeaderValue::from_name(empty_header_name);
    assert_eq!(val, HeaderValue::from_bytes(b"").unwrap());
}

#[test]
fn test_from_name_case_insensitive() {
    use http::header::{HeaderValue, HeaderName};

    let header_name_upper = HeaderName::from_bytes(b"ACCEPT").unwrap();
    let val_upper = HeaderValue::from_name(header_name_upper);
    let header_name_lower = HeaderName::from_bytes(b"accept").unwrap();
    let val_lower = HeaderValue::from_name(header_name_lower);
    assert_eq!(val_upper, val_lower);
}

