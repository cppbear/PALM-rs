// Answer 0

#[test]
fn test_invalid_uri_characters() {
    use http::uri::path::{from_shared, InvalidUri};
    use bytes::Bytes;

    // Given an input that includes invalid characters like 0x25 (which is not allowed)
    let invalid_uri = Bytes::from_static(&[0x21, 0x22, 0x23, 0x25, 0x3D, 0x7B, 0x7C, 0x7E]);

    // When calling from_shared
    let result: Result<_, InvalidUri> = from_shared(invalid_uri);

    // Then it should return an Err with InvalidUriChar kind
    assert!(result.is_err());
}

#[test]
fn test_uri_with_fragment() {
    use http::uri::path::{from_shared, InvalidUri};
    use bytes::Bytes;

    // Given an input that includes a valid fragment indicator
    let uri_with_fragment = Bytes::from_static(&[0x21, 0x22, 0x23, 0x3D, 0x3F, 0x7E, 0x23]);

    // When calling from_shared
    let result: Result<_, InvalidUri> = from_shared(uri_with_fragment);

    // Then it should return an Err with InvalidUriChar kind
    assert!(result.is_err());
}

#[test]
fn test_uri_with_query_and_invalid_characters() {
    use http::uri::path::{from_shared, InvalidUri};
    use bytes::Bytes;

    // Given an invalid input with a query and disallowed characters
    let invalid_query_uri = Bytes::from_static(&[0x21, 0x22, 0x23, 0x3D, 0x7E, 0x3A]);

    // When calling from_shared
    let result: Result<_, InvalidUri> = from_shared(invalid_query_uri);

    // Then it should return an Err with InvalidUriChar kind
    assert!(result.is_err());
}

#[test]
fn test_uri_with_allowed_characters() {
    use http::uri::path::{from_shared, InvalidUri};
    use bytes::Bytes;

    // Given an input that includes allowed characters only
    let valid_uri = Bytes::from_static(&[0x21, 0x24, 0x3B, 0x3D, 0x7E]);

    // When calling from_shared
    let result: Result<_, InvalidUri> = from_shared(valid_uri);

    // Then it should return an Ok
    assert!(result.is_ok());
}

