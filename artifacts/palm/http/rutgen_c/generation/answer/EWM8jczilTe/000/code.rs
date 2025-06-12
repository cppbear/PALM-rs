// Answer 0

#[test]
fn test_from_maybe_shared_bytes() {
    use bytes::Bytes;

    let bytes = Bytes::from_static(b"/path?query=value");
    let path_and_query = PathAndQuery::from_maybe_shared(bytes).unwrap();

    assert_eq!(path_and_query.data.bytes.as_ref(), b"/path?query=value");
    assert_eq!(path_and_query.query, 5); // index of '?'
}

#[test]
fn test_from_maybe_shared_vec_u8() {
    let vec: Vec<u8> = b"/path?query=value".to_vec();
    let path_and_query = PathAndQuery::from_maybe_shared(vec).unwrap();

    assert_eq!(path_and_query.data.bytes.as_ref(), b"/path?query=value");
    assert_eq!(path_and_query.query, 5); // index of '?'
}

#[test]
fn test_from_maybe_shared_array_u8() {
    let array: &[u8; 18] = b"/path?query=value";
    let path_and_query = PathAndQuery::from_maybe_shared(array).unwrap();

    assert_eq!(path_and_query.data.bytes.as_ref(), b"/path?query=value");
    assert_eq!(path_and_query.query, 5); // index of '?'
}

#[test]
fn test_from_maybe_shared_invalid_uri() {
    let invalid_data: &[u8] = b"/path\xFFquery=value";
    let result = PathAndQuery::from_maybe_shared(invalid_data);

    assert!(result.is_err());
}

