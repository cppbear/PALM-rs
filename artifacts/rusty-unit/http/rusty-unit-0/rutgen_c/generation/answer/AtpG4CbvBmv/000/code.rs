// Answer 0

#[test]
fn test_authority_empty() {
    let authority = Authority::empty();
    assert_eq!(authority.data.bytes.len(), 0);
}

#[test]
fn test_authority_from_static() {
    let static_authority = Authority::from_static("example.com");
    assert_eq!(static_authority.as_str(), "example.com");
}

#[test]
fn test_authority_from_shared_valid() {
    use bytes::Bytes;
    let shared_bytes = Bytes::from_static(b"example.com");
    let authority = Authority::from_shared(shared_bytes).unwrap();
    assert_eq!(authority.as_str(), "example.com");
}

#[should_panic]
fn test_authority_from_static_invalid() {
    // Simulating an invalid scenario would usually need an invalid static string
    // Since the method guarantees it can convert valid static strings, we can't really break it here.
    // This test serves as an illustrative placeholder.
    let _ = Authority::from_static("invalid\0string");
}

#[test]
fn test_authority_from_maybe_shared() {
    use bytes::Bytes;
    let authority = Authority::from_maybe_shared(Bytes::from_static(b"example.com")).unwrap();
    assert_eq!(authority.as_str(), "example.com");
}

