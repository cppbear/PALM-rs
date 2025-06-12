// Answer 0

#[test]
fn test_from_shared_too_long_uri() {
    let s = Bytes::from(vec![0u8; MAX_LEN + 1]); // Exceeds the maximum length
    let result = Uri::from_shared(s);
    assert!(result.is_err());
}

#[test]
fn test_from_shared_empty_uri() {
    let s = Bytes::from(vec![]); // Empty input
    let result = Uri::from_shared(s);
    assert!(result.is_err());
}

#[test]
fn test_from_shared_slash_uri() {
    let s = Bytes::from(vec![b'/']); // Single character '/'
    let result = Uri::from_shared(s).unwrap();
    assert_eq!(result.scheme(), Some(&Scheme::empty()));
    assert_eq!(result.authority(), Some(&Authority::empty()));
    assert_eq!(result.path_and_query().as_str(), "/");
}

#[test]
fn test_from_shared_star_uri() {
    let s = Bytes::from(vec![b'*']); // Single character '*'
    let result = Uri::from_shared(s).unwrap();
    assert_eq!(result.scheme(), Some(&Scheme::empty()));
    assert_eq!(result.authority(), Some(&Authority::empty()));
    assert_eq!(result.path_and_query().as_str(), "*");
}

#[test]
fn test_from_shared_valid_authority() {
    let s = Bytes::from_static(b"example.com"); // Valid authority
    let authority = Authority::from_shared(s.clone()).unwrap();
    
    let result = Uri::from_shared(s).unwrap();
    assert_eq!(result.scheme(), Some(&Scheme::empty()));
    assert_eq!(result.authority(), Some(&authority));
    assert_eq!(result.path_and_query().data.as_str(), "");
}

#[test]
fn test_from_shared_slash_path_and_query() {
    let s = Bytes::from_static(b"/path?query=value"); // Valid path with query
    let result = Uri::from_shared(s).unwrap();
    assert_eq!(result.scheme(), Some(&Scheme::empty()));
    assert_eq!(result.authority(), Some(&Authority::empty()));
    assert_eq!(result.path_and_query().as_str(), "/path?query=value");
}

