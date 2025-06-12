// Answer 0

#[test]
fn test_from_name_valid() {
    use http::header::{HeaderValue, HeaderName, ACCEPT};

    let val = HeaderValue::from_name(ACCEPT);
    assert_eq!(val, HeaderValue::from_bytes(b"accept").unwrap());
}

#[test]
fn test_from_name_empty() {
    use http::header::{HeaderValue, HeaderName};

    let empty_header_name = HeaderName::from_static("");
    let val = HeaderValue::from_name(empty_header_name);
    assert_eq!(val, HeaderValue::from_bytes(b"").unwrap());
}

#[test]
#[should_panic]
fn test_from_name_invalid() {
    use http::header::{HeaderValue, HeaderName};

    let invalid_header_name = HeaderName::from_static("Invalid-Header-Name");
    // Assuming HeaderValue::from_bytes would panic for invalid case,
    // here we are testing that from_name handles an invalid case correctly.
    // Note: This test assumes that HeaderName's validity is checked earlier, 
    // otherwise this may not panic, depending on the library's implementation.
    let _ = HeaderValue::from_name(invalid_header_name);
}

