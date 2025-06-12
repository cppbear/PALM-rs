// Answer 0

#[test]
fn test_from_shared_empty() {
    let bytes = Bytes::from_static(b"");
    let result = Uri::from_shared(bytes);
    assert_eq!(result.unwrap_err().kind(), &ErrorKind::Empty);
}

#[test]
fn test_from_shared_too_long() {
    let long_uri = vec![b'a'; MAX_LEN + 1];
    let bytes = Bytes::from(long_uri);
    let result = Uri::from_shared(bytes);
    assert_eq!(result.unwrap_err().kind(), &ErrorKind::TooLong);
}

#[test]
fn test_from_shared_slash() {
    let bytes = Bytes::from_static(b"/");
    let result = Uri::from_shared(bytes).unwrap();
    assert!(result.scheme.is_empty());
    assert!(result.authority.is_empty());
    assert_eq!(result.path_and_query.data.as_str(), "/");
}

#[test]
fn test_from_shared_star() {
    let bytes = Bytes::from_static(b"*");
    let result = Uri::from_shared(bytes).unwrap();
    assert!(result.scheme.is_empty());
    assert!(result.authority.is_empty());
    assert_eq!(result.path_and_query.data.as_str(), "*");
}

#[test]
fn test_from_shared_valid_authority() {
    let bytes = Bytes::from_static(b"example.com");
    let result = Uri::from_shared(bytes).unwrap();
    assert!(result.scheme.is_empty());
    assert!(result.authority.as_str() == "example.com");
}

#[test]
fn test_from_shared_valid_path() {
    let bytes = Bytes::from_static(b"/path/to/resource");
    let result = Uri::from_shared(bytes).unwrap();
    assert!(result.scheme.is_empty());
    assert!(result.authority.is_empty());
    assert_eq!(result.path_and_query.data.as_str(), "/path/to/resource");
}

