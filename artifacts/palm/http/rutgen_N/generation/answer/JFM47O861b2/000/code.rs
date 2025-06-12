// Answer 0

#[test]
fn test_from_shared_empty_string() {
    let bytes = Bytes::from(&b""[..]);
    let result = from_shared(bytes);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::Empty);
}

#[test]
fn test_from_shared_too_long() {
    let bytes = Bytes::from(vec![0; MAX_LEN + 1]);
    let result = from_shared(bytes);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::TooLong);
}

#[test]
fn test_from_shared_single_slash() {
    let bytes = Bytes::from(&b"/"[..]);
    let result = from_shared(bytes).unwrap();
    assert_eq!(result.scheme, Scheme::empty());
    assert_eq!(result.authority, Authority::empty());
    assert_eq!(result.path_and_query, PathAndQuery::slash());
}

#[test]
fn test_from_shared_single_star() {
    let bytes = Bytes::from(&b"*"[..]);
    let result = from_shared(bytes).unwrap();
    assert_eq!(result.scheme, Scheme::empty());
    assert_eq!(result.authority, Authority::empty());
    assert_eq!(result.path_and_query, PathAndQuery::star());
}

#[test]
fn test_from_shared_with_authority() {
    let bytes = Bytes::from(&b"example.com"[..]);
    let result = from_shared(bytes).unwrap();
    assert_eq!(result.scheme, Scheme::empty());
    assert_eq!(result.authority, Authority::from_shared(bytes).unwrap());
    assert_eq!(result.path_and_query, PathAndQuery::empty());
}

#[test]
fn test_from_shared_path_with_query() {
    let bytes = Bytes::from(&b"/path?query"[..]);
    let result = from_shared(bytes).unwrap();
    assert_eq!(result.scheme, Scheme::empty());
    assert_eq!(result.authority, Authority::empty());
    assert_eq!(result.path_and_query, PathAndQuery::from_shared(bytes).unwrap());
}

