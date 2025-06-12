// Answer 0

#[test]
fn test_from_shared_valid_path() {
    let input = Bytes::from_static(b"/path/to/resource");
    let result = PathAndQuery::from_shared(input).unwrap();
    assert_eq!(result.data.bytes, Bytes::from_static(b"/path/to/resource"));
    assert_eq!(result.query, NONE);
}

#[test]
fn test_from_shared_with_query() {
    let input = Bytes::from_static(b"/path/to/resource?query=1");
    let result = PathAndQuery::from_shared(input).unwrap();
    assert_eq!(result.data.bytes, Bytes::from_static(b"/path/to/resource"));
    assert_eq!(result.query, 20); // '?' appears at index 20
}

#[test]
fn test_from_shared_with_invalid_character() {
    let input = Bytes::from_static(b"/path/to/resource\xFF");
    let result = PathAndQuery::from_shared(input);
    assert!(result.is_err());
    if let Err(InvalidUri(kind)) = result {
        assert_eq!(kind, ErrorKind::InvalidUriChar);
    }
}

#[test]
fn test_from_shared_empty_path() {
    let input = Bytes::from_static(b"");
    let result = PathAndQuery::from_shared(input);
    assert!(result.is_err());
    if let Err(InvalidUri(kind)) = result {
        assert_eq!(kind, ErrorKind::PathAndQueryMissing);
    }
}

#[test]
fn test_from_shared_with_fragment() {
    let input = Bytes::from_static(b"/path/to/resource#fragment");
    let result = PathAndQuery::from_shared(input).unwrap();
    assert_eq!(result.data.bytes, Bytes::from_static(b"/path/to/resource"));
    assert_eq!(result.query, NONE);
}

