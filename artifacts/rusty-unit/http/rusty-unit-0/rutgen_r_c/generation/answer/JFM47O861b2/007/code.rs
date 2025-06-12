// Answer 0

#[test]
fn test_from_shared_empty_bytes() {
    let result = Uri::from_shared(Bytes::new());
    assert_eq!(result, Err(InvalidUri(ErrorKind::Empty)));
}

#[test]
fn test_from_shared_too_long() {
    let long_bytes = Bytes::from(vec![0u8; MAX_LEN + 1]);
    let result = Uri::from_shared(long_bytes);
    assert_eq!(result, Err(InvalidUri(ErrorKind::TooLong)));
}

#[test]
fn test_from_shared_single_slash() {
    let bytes = Bytes::from(vec![b'/']);
    let result = Uri::from_shared(bytes).unwrap();
    assert_eq!(result.scheme(), Some(&Scheme::empty()));
    assert_eq!(result.authority(), Some(&Authority::empty()));
    assert_eq!(result.path_and_query(), Some(&PathAndQuery::slash()));
}

#[test]
fn test_from_shared_single_star() {
    let bytes = Bytes::from(vec![b'*']);
    let result = Uri::from_shared(bytes).unwrap();
    assert_eq!(result.scheme(), Some(&Scheme::empty()));
    assert_eq!(result.authority(), Some(&Authority::empty()));
    assert_eq!(result.path_and_query(), Some(&PathAndQuery::star()));
}

#[test]
fn test_from_shared_valid_authority() {
    let bytes = Bytes::from_static("example.com");
    let result = Uri::from_shared(bytes).unwrap();
    assert_eq!(result.scheme(), Some(&Scheme::empty()));
    assert_eq!(result.authority().unwrap().as_str(), "example.com");
    assert_eq!(result.path_and_query(), Some(&PathAndQuery::empty()));
}

