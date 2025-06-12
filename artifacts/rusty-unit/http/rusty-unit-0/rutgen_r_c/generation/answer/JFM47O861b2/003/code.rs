// Answer 0

#[test]
fn test_from_shared_empty() {
    let input = Bytes::from_static(b"");
    let result = Uri::from_shared(input);
    assert_eq!(result, Err(InvalidUri(ErrorKind::Empty)));
}

#[test]
fn test_from_shared_single_slash() {
    let input = Bytes::from_static(b"/");
    let result = Uri::from_shared(input);
    assert_eq!(result, Ok(Uri {
        scheme: Scheme::empty(),
        authority: Authority::empty(),
        path_and_query: PathAndQuery::slash(),
    }));
}

#[test]
fn test_from_shared_single_star() {
    let input = Bytes::from_static(b"*");
    let result = Uri::from_shared(input);
    assert_eq!(result, Ok(Uri {
        scheme: Scheme::empty(),
        authority: Authority::empty(),
        path_and_query: PathAndQuery::star(),
    }));
}

#[test]
fn test_from_shared_valid_authority() {
    let input = Bytes::from_static(b"example.com");
    let result = Uri::from_shared(input);
    assert!(result.is_ok());
    let uri = result.unwrap();
    assert_eq!(uri.scheme(), None);
    assert_eq!(uri.authority().unwrap().as_str(), "example.com");
    assert_eq!(uri.path_and_query().unwrap().as_str(), "");
}

#[test]
fn test_from_shared_path_starting_with_slash() {
    let input = Bytes::from_static(b"/path/to/resource");
    let result = Uri::from_shared(input);
    assert!(result.is_ok());
    let uri = result.unwrap();
    assert_eq!(uri.scheme(), None);
    assert_eq!(uri.path_and_query().unwrap().as_str(), "/path/to/resource");
}

#[test]
fn test_from_shared_valid_path_and_query() {
    let input = Bytes::from_static(b"/path?query=value");
    let result = Uri::from_shared(input);
    assert!(result.is_ok());
    let uri = result.unwrap();
    assert_eq!(uri.path_and_query().unwrap().as_str(), "/path?query=value");
}

#[test]
fn test_from_shared_too_long() {
    let input = Bytes::from(vec![b'a'; MAX_LEN + 1]);
    let result = Uri::from_shared(input);
    assert_eq!(result, Err(InvalidUri(ErrorKind::TooLong)));
}

