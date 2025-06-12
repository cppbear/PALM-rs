// Answer 0

#[test]
fn test_from_shared_invalid_uri_char() {
    use bytes::Bytes;
    
    let invalid_chars = vec![
        b'\xFF', // Outside valid range
        b'_',    // In range (b'0x40..=0x5F')
        b'a',    // In range (b'0x61..=0x7A')
        b'|',    // Not valid in URI
        b'~',    // Valid, should not cause an error
        b'@',    // In range (b'0x40..=0x5F')
    ];
    
    for &invalid in &invalid_chars {
        let bytes = Bytes::from_static(b"/path/with/invalid") + Bytes::from_static(&[invalid]);
        let result = PathAndQuery::from_shared(bytes);
        assert_eq!(result, Err(ErrorKind::InvalidUriChar.into()));
    }
}

#[test]
fn test_from_shared_valid_uri_characters() {
    use bytes::Bytes;

    let valid_bytes = Bytes::from_static(b"/path/to/resource?query=value#fragment");
    let result = PathAndQuery::from_shared(valid_bytes);
    assert!(result.is_ok());
    
    let path_and_query = result.unwrap();
    assert_eq!(path_and_query.data.bytes, Bytes::from_static(b"/path/to/resource"));
    assert_eq!(path_and_query.query, 24); // The index of '?'
}

#[test]
fn test_from_shared_fragment_character() {
    use bytes::Bytes;

    let valid_bytes = Bytes::from_static(b"/path/to/resource#fragment");
    let result = PathAndQuery::from_shared(valid_bytes);
    assert!(result.is_ok());

    let path_and_query = result.unwrap();
    assert_eq!(path_and_query.data.bytes, Bytes::from_static(b"/path/to/resource"));
    assert_eq!(path_and_query.query, u16::MAX); // No query present
}

#[test]
fn test_from_shared_special_characters() {
    use bytes::Bytes;

    let valid_bytes = Bytes::from_static(b"/path/with/{curly}and\"quotes\"");
    let result = PathAndQuery::from_shared(valid_bytes);
    assert!(result.is_ok());

    let path_and_query = result.unwrap();
    assert_eq!(path_and_query.data.bytes, Bytes::from_static(b"/path/with/{curly}and\"quotes\""));
    assert_eq!(path_and_query.query, u16::MAX); // No query present
}

#[test]
fn test_from_shared_invalid_character_at_boundary() {
    use bytes::Bytes;

    let invalid_boundary = Bytes::from_static(b"/path/with/invalid\xFF");
    let result = PathAndQuery::from_shared(invalid_boundary);
    assert_eq!(result, Err(ErrorKind::InvalidUriChar.into()));
}

