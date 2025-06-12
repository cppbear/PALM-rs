// Answer 0

#[test]
fn test_from_shared_empty_bytes() {
    let result = Uri::from_shared(Bytes::from_static(b""));
    assert_eq!(result, Err(InvalidUri(ErrorKind::Empty)));
}

#[test]
fn test_from_shared_too_long() {
    let result = Uri::from_shared(Bytes::from_static(&[0; MAX_LEN + 1]));
    assert_eq!(result, Err(InvalidUri(ErrorKind::TooLong)));
}

#[test]
fn test_from_shared_single_slash() {
    let result = Uri::from_shared(Bytes::from_static(b"/"));
    let expected = Uri {
        scheme: Scheme::empty(),
        authority: Authority::empty(),
        path_and_query: PathAndQuery::slash(),
    };
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_from_shared_single_asterisk() {
    let result = Uri::from_shared(Bytes::from_static(b"*"));
    let expected = Uri {
        scheme: Scheme::empty(),
        authority: Authority::empty(),
        path_and_query: PathAndQuery::star(),
    };
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_from_shared_authority() {
    let result = Uri::from_shared(Bytes::from_static(b"example.com"));
    let authority = Authority::from_static("example.com");
    let expected = Uri {
        scheme: Scheme::empty(),
        authority,
        path_and_query: PathAndQuery::empty(),
    };
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_from_shared_path() {
    let result = Uri::from_shared(Bytes::from_static(b"/path?query"));
    let path_and_query = PathAndQuery::from_static("/path?query");
    let expected = Uri {
        scheme: Scheme::empty(),
        authority: Authority::empty(),
        path_and_query,
    };
    assert_eq!(result, Ok(expected));
}

