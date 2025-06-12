// Answer 0

#[test]
fn test_authority_as_str_empty() {
    let authority = Authority::empty();
    assert_eq!(authority.as_str(), "");
}

#[test]
fn test_authority_as_str_static() {
    let authority = Authority::from_static("example.com");
    assert_eq!(authority.as_str(), "example.com");
}

#[test]
fn test_authority_as_str_maybe_shared() {
    let bytes = Bytes::from_static(b"test.com");
    let authority = Authority::from_maybe_shared(bytes).expect("Failed to create Authority");
    assert_eq!(authority.as_str(), "test.com");
}

#[test]
fn test_authority_as_str_maybe_shared_vec() {
    let authority = Authority::from_maybe_shared(vec![b'a', b'b', b'c']).expect("Failed to create Authority");
    assert_eq!(authority.as_str(), "abc");
}

