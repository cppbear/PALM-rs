// Answer 0

#[test]
fn test_from_shared_with_valid_path_and_query() {
    use bytes::Bytes;

    // Input that has both a valid path and a query string, and avoids invalid characters
    let input: Bytes = Bytes::from_static(b"/path/to/resource?query=1");
    let result = PathAndQuery::from_shared(input);

    assert!(result.is_ok());
    if let Ok(path_and_query) = result {
        assert_eq!(path_and_query.query, 22); // index of '?' in the input
        assert_eq!(path_and_query.data.bytes.as_ref(), b"/path/to/resource");
    }
}

#[test]
fn test_from_shared_with_valid_path_and_fragment() {
    use bytes::Bytes;

    // Input that contains a fragment, which should be removed correctly
    let input: Bytes = Bytes::from_static(b"/path/to/resource#fragment");
    let result = PathAndQuery::from_shared(input);

    assert!(result.is_ok());
    if let Ok(path_and_query) = result {
        assert_eq!(path_and_query.query, u16::MAX); // No query, so it should be unchanged
        assert_eq!(path_and_query.data.bytes.as_ref(), b"/path/to/resource");
    }
}

#[test]
fn test_from_shared_with_embedded_json() {
    use bytes::Bytes;

    // Input that contains valid characters, including those normally requiring percent-encoding
    let input: Bytes = Bytes::from_static(b"/path/with/json{\"key\":\"value\"}#fragment");
    let result = PathAndQuery::from_shared(input);

    assert!(result.is_ok());
    if let Ok(path_and_query) = result {
        assert_eq!(path_and_query.query, u16::MAX); // No query present
        assert_eq!(path_and_query.data.bytes.as_ref(), b"/path/with/json{\"key\":\"value\"}");
    }
}

#[test]
fn test_from_shared_with_invalid_characters() {
    use bytes::Bytes;

    // Input with invalid characters which should return an error
    let input: Bytes = Bytes::from_static(b"/path/to/\xFF"); // 0xFF is an invalid character
    let result = PathAndQuery::from_shared(input);

    assert!(result.is_err());
    if let Err(InvalidUri(ErrorKind::InvalidUriChar)) = result {
        // Test passes because we expect this error
    } else {
        panic!("Expected InvalidUriChar error")
    }
}

