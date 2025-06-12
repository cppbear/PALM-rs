// Answer 0

#[test]
fn test_from_shared_with_valid_uri_and_fragment() {
    use bytes::Bytes;
    use crate::uri::path::{from_shared, PathAndQuery, InvalidUri};
    
    // Test input simulating a valid URI with a fragment.
    let input_bytes: Bytes = Bytes::from_static(b"/path/to/resource#fragment");
    
    let result: Result<PathAndQuery, InvalidUri> = from_shared(input_bytes);
    
    // Check if the result is Ok and contains the expected values.
    assert!(result.is_ok());
    if let Ok(path_and_query) = result {
        assert_eq!(path_and_query.query, 0); // expect query to be NONE
        assert_eq!(path_and_query.data.as_ref(), b"/path/to/resource");
    }
}

#[test]
fn test_from_shared_with_non_utf8_bytes_and_valid_fragment() {
    use bytes::Bytes;
    use crate::uri::path::{from_shared, PathAndQuery, InvalidUri};
    
    // Test input including non-UTF8 bytes and a fragment.
    let input_bytes: Bytes = Bytes::from_static(b"/path/\xFFresource#fragment"); // includes 0xFF (non-UTF8)
    
    let result: Result<PathAndQuery, InvalidUri> = from_shared(input_bytes);
    
    assert!(result.is_ok());
    if let Ok(path_and_query) = result {
        assert_eq!(path_and_query.query, 0); // expect query to be NONE
        assert_eq!(path_and_query.data.as_ref(), b"/path/\xFFresource");
    }
}

#[test]
fn test_from_shared_with_invalid_character() {
    use bytes::Bytes;
    use crate::uri::path::{from_shared, InvalidUri};
    
    // Test input containing an invalid character not allowed in URI path.
    let input_bytes: Bytes = Bytes::from_static(b"/path/to/invalid\x80char#fragment"); // includes 0x80 (invalid)
    
    let result: Result<PathAndQuery, InvalidUri> = from_shared(input_bytes);
    
    // Expect an error result due to invalid character
    assert!(result.is_err());
}

