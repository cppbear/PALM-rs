// Answer 0

#[test]
fn test_from_shared_with_valid_utf8_and_query() {
    use bytes::Bytes;
    use crate::uri::path::{from_shared, InvalidUri};

    let input = Bytes::from_static(b"/path/to/resource?query=value#fragment");
    let result: Result<_, InvalidUri> = from_shared(input);
    assert!(result.is_ok());
}

#[test]
fn test_from_shared_with_special_characters() {
    use bytes::Bytes;
    use crate::uri::path::{from_shared, InvalidUri};

    let input = Bytes::from_static(b"/path/to/resource?query=value&other=123#{\"key\": \"value\"}");
    let result: Result<_, InvalidUri> = from_shared(input);
    assert!(result.is_ok());
}

#[test]
fn test_from_shared_with_only_path_and_fragment() {
    use bytes::Bytes;
    use crate::uri::path::{from_shared, InvalidUri};

    let input = Bytes::from_static(b"/path/to/resource#fragment");
    let result: Result<_, InvalidUri> = from_shared(input);
    assert!(result.is_ok());
}

#[test]
fn test_from_shared_with_invalid_character() {
    use bytes::Bytes;
    use crate::uri::path::{from_shared, InvalidUri};

    let input = Bytes::from_static(b"/path/to/resource\xFF"); // Invalid character
    let result: Result<_, InvalidUri> = from_shared(input);
    assert!(result.is_err());
}

#[test]
fn test_from_shared_with_percent_encoded_value() {
    use bytes::Bytes;
    use crate::uri::path::{from_shared, InvalidUri};

    let input = Bytes::from_static(b"/path/to/resource%20with%20spaces?query=value");
    let result: Result<_, InvalidUri> = from_shared(input);
    assert!(result.is_ok());
}

