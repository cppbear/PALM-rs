// Answer 0

#[test]
fn test_from_maybe_shared_with_bytes() {
    use bytes::Bytes;

    let shared_bytes = Bytes::from_static(b"test?query=value");
    let result = PathAndQuery::from_maybe_shared(shared_bytes);
    assert!(result.is_ok());
    let path_and_query = result.unwrap();
    assert_eq!(path_and_query.query, 4); // '?' is at index 4
    assert_eq!(path_and_query.data.bytes, Bytes::from_static(b"test")); // Path should be "test"
}

#[test]
fn test_from_maybe_shared_with_vec_u8() {
    let vec_bytes: Vec<u8> = b"another/test?query=value".to_vec();
    let result = PathAndQuery::from_maybe_shared(vec_bytes);
    assert!(result.is_ok());
    let path_and_query = result.unwrap();
    assert_eq!(path_and_query.query, 6); // '?' is at index 6
    assert_eq!(path_and_query.data.bytes, Bytes::from_static(b"another/test")); // Path should be "another/test"
}

#[test]
fn test_from_maybe_shared_with_slice() {
    let slice_bytes: &[u8] = b"slice/test#fragment";
    let result = PathAndQuery::from_maybe_shared(slice_bytes);
    assert!(result.is_ok());
    let path_and_query = result.unwrap();
    assert_eq!(path_and_query.query, 5); // '?' is at index 5 (not present, should be NONE)
    assert_eq!(path_and_query.data.bytes, Bytes::from_static(b"slice/test")); // Path should be "slice/test"
}

#[test]
#[should_panic]
fn test_from_maybe_shared_invalid_char() {
    let invalid_bytes: &[u8] = b"invalid\xFFbytes";
    let _ = PathAndQuery::from_maybe_shared(invalid_bytes).unwrap();
}

#[test]
fn test_from_maybe_shared_empty() {
    let empty_bytes: &[u8] = &[];
    let result = PathAndQuery::from_maybe_shared(empty_bytes);
    assert!(result.is_ok());
    let path_and_query = result.unwrap();
    assert_eq!(path_and_query.query, u16::MAX); // No query should be present
    assert_eq!(path_and_query.data.bytes.len(), 0); // Path should be empty
}

