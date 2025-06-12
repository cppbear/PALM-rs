// Answer 0

#[test]
fn test_from_shared_empty() {
    let input = Bytes::from_static(&[]);
    
    let result = Uri::from_shared(input);
    
    assert_eq!(result.is_err(), true);
    assert_eq!(result.err().unwrap().0, ErrorKind::Empty);
}

#[test]
fn test_from_shared_single_slash() {
    let input = Bytes::from_static(&[b'/']);
    
    let result = Uri::from_shared(input).unwrap();
    
    assert_eq!(result.path_and_query.as_str(), "/");
    assert!(result.scheme.as_str().is_empty());
}

#[test]
fn test_from_shared_single_star() {
    let input = Bytes::from_static(&[b'*']);
    
    let result = Uri::from_shared(input).unwrap();
    
    assert_eq!(result.path_and_query.as_str(), "*");
    assert!(result.scheme.as_str().is_empty());
}

#[test]
fn test_from_shared_single_invalid() {
    let input = Bytes::from_static(&[b'A']);
    
    let result = Uri::from_shared(input);
    
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.as_ref().unwrap().authority.as_str(), "A");
}

#[test]
fn test_from_shared_max_length() {
    let input = Bytes::from_static(&[b'a'; MAX_LEN]);
    
    let result = Uri::from_shared(input);
    
    assert_eq!(result.is_err(), true);
    assert_eq!(result.err().unwrap().0, ErrorKind::TooLong);
}

#[test]
fn test_from_shared_invalid_chars() {
    let input = Bytes::from_static(&[b'%', b'&', b'#']);
    
    let result = Uri::from_shared(input);
    
    assert_eq!(result.is_err(), true);
    assert_eq!(result.err().unwrap().0, ErrorKind::InvalidUriChar);
}

#[test]
fn test_from_shared_slash_with_invalid_rights() {
    let input = Bytes::from_static(b"/invalid/path");
    
    let result = Uri::from_shared(input).unwrap();
    
    assert_eq!(result.path_and_query.as_str(), "/invalid/path");
    assert!(result.scheme.as_str().is_empty());
}

