// Answer 0

#[test]
fn test_create_authority_valid() {
    struct MockBytes<'a>(&'a [u8]);

    let input = MockBytes(b"username:password@host:port");
    let result = create_authority(input, |b| Bytes::from(b.0));
    
    assert!(result.is_ok());
    let authority = result.unwrap();
    assert_eq!(authority.data.to_string(), "username:password@host:port");
}

#[test]
#[should_panic]
fn test_create_authority_invalid_utf8() {
    struct MockBytes<'a>(&'a [u8]);

    let input = MockBytes(b"\xff");
    let _ = create_authority(input, |b| Bytes::from(b.0));
}

#[test]
fn test_create_authority_empty() {
    struct MockBytes<'a>(&'a [u8]);

    let input = MockBytes(b"");
    let result = create_authority(input, |b| Bytes::from(b.0));
    
    assert!(result.is_err());
}

#[test]
fn test_create_authority_boundary() {
    struct MockBytes<'a>(&'a [u8]);

    let input = MockBytes(b"valid@host");
    let result = create_authority(input, |b| Bytes::from(b.0));
    
    assert!(result.is_ok());
    let authority = result.unwrap();
    assert_eq!(authority.data.to_string(), "valid@host");
}

