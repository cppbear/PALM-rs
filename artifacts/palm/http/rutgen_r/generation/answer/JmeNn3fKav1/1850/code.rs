// Answer 0

fn test_from_shared_valid_path_with_query() {
    use http::{Bytes, InvalidUri, PathAndQuery, from_shared};

    // Test input that has a query and valid characters in path
    let input = Bytes::from_static(b"/path/to/resource?query=value");
    let result: Result<PathAndQuery, InvalidUri> = from_shared(input);

    assert!(result.is_ok());

    if let Ok(path_and_query) = result {
        assert_eq!(path_and_query.data, "/path/to/resource");
        assert_eq!(path_and_query.query, 25); // The index of '?' in the input
    }
}

fn test_from_shared_valid_path_with_fragment() {
    use http::{Bytes, InvalidUri, PathAndQuery, from_shared};

    // Test input that has a fragment and valid characters in path
    let input = Bytes::from_static(b"/path/to/resource#fragment");
    let result: Result<PathAndQuery, InvalidUri> = from_shared(input);

    assert!(result.is_ok());

    if let Ok(path_and_query) = result {
        assert_eq!(path_and_query.data, "/path/to/resource");
        assert_eq!(path_and_query.query, 0); // No query present
    }
}

fn test_from_shared_valid_path_with_query_and_fragment() {
    use http::{Bytes, InvalidUri, PathAndQuery, from_shared};

    // Test input that has both a query and a fragment
    let input = Bytes::from_static(b"/path/to/resource?query=value#fragment");
    let result: Result<PathAndQuery, InvalidUri> = from_shared(input);

    assert!(result.is_ok());

    if let Ok(path_and_query) = result {
        assert_eq!(path_and_query.data, "/path/to/resource");
        assert_eq!(path_and_query.query, 25); // The index of '?' in the input
    }
}

fn test_from_shared_invalid_path_character() {
    use http::{Bytes, InvalidUri, PathAndQuery, from_shared};
    
    // Test input with an invalid character in the path
    let input = Bytes::from_static(b"/path/to/invalid\xFFchar");
    let result: Result<PathAndQuery, InvalidUri> = from_shared(input);

    assert!(result.is_err());
}

fn test_from_shared_utf8_query() {
    use http::{Bytes, InvalidUri, PathAndQuery, from_shared};
    
    // Test input with potentially invalid UTF-8 characters in query
    let input = Bytes::from_static(b"/path/to/resource?query=\xC3\x28"); // Invalid UTF-8
    let result: Result<PathAndQuery, InvalidUri> = from_shared(input);

    assert!(result.is_err());
}

