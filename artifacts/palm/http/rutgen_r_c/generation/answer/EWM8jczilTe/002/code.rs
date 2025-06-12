// Answer 0

#[test]
fn test_from_maybe_shared_bytes() {
    use bytes::Bytes;
    let input = Bytes::from_static(b"/path?query=value");
    let result = PathAndQuery::from_maybe_shared(input);
    assert!(result.is_ok());
    let path_and_query = result.unwrap();
    assert_eq!(path_and_query.path(), "/path");
    assert_eq!(path_and_query.query(), Some("query=value"));
}

#[test]
fn test_from_maybe_shared_slice() {
    let input: &[u8] = b"/path?query=value";
    let result = PathAndQuery::from_maybe_shared(input);
    assert!(result.is_ok());
    let path_and_query = result.unwrap();
    assert_eq!(path_and_query.path(), "/path");
    assert_eq!(path_and_query.query(), Some("query=value"));
}

#[test]
fn test_from_maybe_shared_invalid_char() {
    let input: &[u8] = b"/path\xFFquery=value"; // Invalid character
    let result = PathAndQuery::from_maybe_shared(input);
    assert!(result.is_err());
}

#[test]
fn test_from_maybe_shared_empty() {
    let input: &[u8] = b""; // Empty input
    let result = PathAndQuery::from_maybe_shared(input);
    assert!(result.is_ok());
    let path_and_query = result.unwrap();
    assert_eq!(path_and_query.path(), "");
    assert!(path_and_query.query().is_none());
}

#[test]
fn test_from_maybe_shared_no_query() {
    let input: &[u8] = b"/path"; // No query string
    let result = PathAndQuery::from_maybe_shared(input);
    assert!(result.is_ok());
    let path_and_query = result.unwrap();
    assert_eq!(path_and_query.path(), "/path");
    assert!(path_and_query.query().is_none());
}

