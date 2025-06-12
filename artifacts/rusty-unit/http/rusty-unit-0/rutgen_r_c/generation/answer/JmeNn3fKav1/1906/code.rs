// Answer 0

#[test]
fn test_from_shared_with_invalid_uri_character() {
    use bytes::Bytes;
    use crate::{PathAndQuery, InvalidUri, ErrorKind};

    // Test input with invalid characters that should return Err
    let invalid_bytes: Bytes = Bytes::from_static(b"hello\xFFworld?query=value#fragment");
    let result: Result<PathAndQuery, InvalidUri> = PathAndQuery::from_shared(invalid_bytes);
    assert_eq!(result, Err(ErrorKind::InvalidUriChar.into()));
}

#[test]
fn test_from_shared_with_valid_uri() {
    use bytes::Bytes;
    use crate::{PathAndQuery, InvalidUri, ErrorKind};

    // Test input that is valid and should return Ok
    let valid_bytes: Bytes = Bytes::from_static(b"/path?query=value#fragment");
    let result: Result<PathAndQuery, InvalidUri> = PathAndQuery::from_shared(valid_bytes);
    assert!(result.is_ok());
}

#[test]
fn test_from_shared_with_query_character() {
    use bytes::Bytes;
    use crate::{PathAndQuery, InvalidUri, ErrorKind};

    // Test input that matches query condition
    let test_bytes: Bytes = Bytes::from_static(b"/path?query=value");
    let result: Result<PathAndQuery, InvalidUri> = PathAndQuery::from_shared(test_bytes);
    assert!(result.is_ok());
    
    // Check if the query is set properly
    if let Ok(path_and_query) = result {
        assert_eq!(path_and_query.query, 5); // '?' is at index 5
    }
}

#[test]
fn test_from_shared_with_invalid_character_after_query() {
    use bytes::Bytes;
    use crate::{PathAndQuery, InvalidUri, ErrorKind};

    // Test input where a character after a valid query is invalid
    let invalid_query_bytes: Bytes = Bytes::from_static(b"/path?query=\xFF");
    let result: Result<PathAndQuery, InvalidUri> = PathAndQuery::from_shared(invalid_query_bytes);
    assert_eq!(result, Err(ErrorKind::InvalidUriChar.into()));
}

#[test]
fn test_from_shared_with_only_fragment() {
    use bytes::Bytes;
    use crate::{PathAndQuery, InvalidUri, ErrorKind};

    // Test input where only fragment is present
    let fragment_bytes: Bytes = Bytes::from_static(b"/path#fragment");
    let result: Result<PathAndQuery, InvalidUri> = PathAndQuery::from_shared(fragment_bytes);
    assert!(result.is_ok());
    
    // Check if the data is set and query is NONE
    if let Ok(path_and_query) = result {
        assert_eq!(path_and_query.query, u16::MAX); // Query should be NONE (u16::MAX)
    }
}

