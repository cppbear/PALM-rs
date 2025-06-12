// Answer 0

#[test]
fn test_from_shared_valid_path() {
    use bytes::Bytes;
    let input = Bytes::from_static(b"/path/to/resource");
    let result = PathAndQuery::from_shared(input);
    assert!(result.is_ok());
    let path_and_query = result.unwrap();
    assert_eq!(path_and_query.query, u16::MAX);
}

#[test]
fn test_from_shared_valid_path_with_query() {
    use bytes::Bytes;
    let input = Bytes::from_static(b"/path/to/resource?query=1");
    let result = PathAndQuery::from_shared(input);
    assert!(result.is_ok());
    let path_and_query = result.unwrap();
    assert_eq!(path_and_query.query, 18); // Position of '?' in the byte array
}

#[test]
fn test_from_shared_with_invalid_uri_char() {
    use bytes::Bytes;
    let input = Bytes::from_static(b"/path/to/resource\xFF");
    let result = PathAndQuery::from_shared(input);
    assert!(result.is_err());
}

#[test]
fn test_from_shared_with_fragment() {
    use bytes::Bytes;
    let input = Bytes::from_static(b"/path/to/resource#fragment");
    let result = PathAndQuery::from_shared(input);
    assert!(result.is_ok());
    let path_and_query = result.unwrap();
    assert_eq!(path_and_query.query, u16::MAX); // No query, should still be max
}

#[test]
fn test_from_shared_invalid_percent_encoded_char() {
    use bytes::Bytes;
    let input = Bytes::from_static(b"/path/to/resource%?");
    let result = PathAndQuery::from_shared(input);
    assert!(result.is_err());
}

#[test]
fn test_from_shared_empty() {
    use bytes::Bytes;
    let input = Bytes::from_static(b"");
    let result = PathAndQuery::from_shared(input);
    assert!(result.is_ok());
    let path_and_query = result.unwrap();
    assert_eq!(path_and_query.data.bytes.len(), 0);
    assert_eq!(path_and_query.query, u16::MAX);
}

