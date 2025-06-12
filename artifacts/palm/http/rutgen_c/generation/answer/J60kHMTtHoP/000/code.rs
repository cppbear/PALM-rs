// Answer 0

#[test]
fn test_from_maybe_shared_with_bytes() {
    use bytes::Bytes;

    let bytes = Bytes::from_static(b"https://example.com");
    let uri = Uri::from_maybe_shared(bytes).unwrap();
    assert_eq!(uri.scheme_str(), Some("https"));
    assert_eq!(uri.authority().unwrap().host(), Some("example.com"));
}

#[test]
fn test_from_maybe_shared_with_slice() {
    let slice: &[u8] = b"https://example.com";
    let uri = Uri::from_maybe_shared(slice).unwrap();
    assert_eq!(uri.scheme_str(), Some("https"));
    assert_eq!(uri.authority().unwrap().host(), Some("example.com"));
}

#[test]
fn test_from_maybe_shared_with_invalid_bytes() {
    let invalid_bytes: &[u8] = b"invalid_uri";
    let result = Uri::from_maybe_shared(invalid_bytes);
    assert!(result.is_err());
}

#[test]
#[should_panic(expected = "static str is not valid URI")]
fn test_from_static_with_invalid_uri() {
    let invalid_uri = Uri::from_static("invalid_uri");
}

