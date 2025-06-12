// Answer 0

#[test]
fn test_from_shared_empty() {
    let input = Bytes::from_static("");
    let result: Result<Uri, InvalidUri> = Uri::from_shared(input);
    assert_eq!(result, Err(InvalidUri(ErrorKind::Empty)));
}

#[test]
fn test_from_shared_single_slash() {
    let input = Bytes::from_static("/");
    let result: Result<Uri, InvalidUri> = Uri::from_shared(input);
    let expected = Uri {
        scheme: Scheme::empty(),
        authority: Authority::empty(),
        path_and_query: PathAndQuery::slash(),
    };
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_from_shared_single_asterisk() {
    let input = Bytes::from_static("*");
    let result: Result<Uri, InvalidUri> = Uri::from_shared(input);
    let expected = Uri {
        scheme: Scheme::empty(),
        authority: Authority::empty(),
        path_and_query: PathAndQuery::star(),
    };
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_from_shared_authority() {
    let input = Bytes::from_static("example.com");
    let result: Result<Uri, InvalidUri> = Uri::from_shared(input);
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
    let input = Bytes::from_static("/path/to/resource");
    let result: Result<Uri, InvalidUri> = Uri::from_shared(input);
    let path_and_query = PathAndQuery::from_static("/path/to/resource");
    let expected = Uri {
        scheme: Scheme::empty(),
        authority: Authority::empty(),
        path_and_query,
    };
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_from_shared_too_long() {
    let input = Bytes::from_static(&vec![b'a'; MAX_LEN + 1]);
    let result: Result<Uri, InvalidUri> = Uri::from_shared(input);
    assert_eq!(result, Err(InvalidUri(ErrorKind::TooLong)));
}

