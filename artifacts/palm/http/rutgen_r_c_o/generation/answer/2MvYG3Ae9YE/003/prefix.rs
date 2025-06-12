// Answer 0

#[test]
fn test_from_lowercase_valid_headers() {
    let _ = HeaderName::from_lowercase(b"content-length");
    let _ = HeaderName::from_lowercase(b"content-type");
    let _ = HeaderName::from_lowercase(b"abcdefg");
    let _ = HeaderName::from_lowercase(b"abcdefghijklmnoqrstuvwxyz");
    let _ = HeaderName::from_lowercase(b"abcdefghijklmnopqrstuvwxyz1234567890");
    let _ = HeaderName::from_lowercase(b"http");
    let _ = HeaderName::from_lowercase(b"hello-world");
}

#[test]
fn test_from_lowercase_invalid_headers() {
    let _ = HeaderName::from_lowercase(b"&%$");
    let _ = HeaderName::from_lowercase(b"");
    assert!(HeaderName::from_lowercase(b"CONTENTS").is_err());
    assert!(HeaderName::from_lowercase(b"aA").is_err());
}

#[test]
fn test_from_lowercase_edge_case() {
    let _ = HeaderName::from_lowercase(b"aaaaaa");
}

#[test]
#[should_panic]
fn test_from_lowercase_panic_condition() {
    // Create input to trigger panic
    HeaderName::from_lowercase(b"invalid-characters: \x80");
}

