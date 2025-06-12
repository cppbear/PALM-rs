// Answer 0

#[test]
fn test_from_shared_with_valid_path_and_query() {
    use crate::uri::path::{from_shared, PathAndQuery};
    use bytes::Bytes;
    use crate::ErrorKind;

    let input = Bytes::from_static(b"/path/to/resource?query=value");
    let result = from_shared(input).unwrap();

    assert!(result.data.to_string() == "/path/to/resource");
    assert!(result.query == 22);
}

#[test]
fn test_from_shared_with_only_path() {
    use crate::uri::path::{from_shared, PathAndQuery};
    use bytes::Bytes;

    let input = Bytes::from_static(b"/path/to/resource");
    let result = from_shared(input).unwrap();

    assert!(result.data.to_string() == "/path/to/resource");
    assert!(result.query == NONE);
}

#[test]
fn test_from_shared_with_fragment() {
    use crate::uri::path::{from_shared, PathAndQuery};
    use bytes::Bytes;

    let input = Bytes::from_static(b"/path/to/resource#fragment");
    let result = from_shared(input).unwrap();

    assert!(result.data.to_string() == "/path/to/resource");
    assert!(result.query == NONE);
}

#[test]
fn test_from_shared_with_non_utf8_bytes() {
    use crate::uri::path::{from_shared};
    use bytes::Bytes;

    let input = Bytes::from_static(b"/path/to/resource\xFF");
    let result = from_shared(input).unwrap_err();

    assert!(matches!(result, ErrorKind::InvalidUriChar));
}

#[test]
fn test_from_shared_with_invalid_characters() {
    use crate::uri::path::{from_shared};
    use bytes::Bytes;

    let input = Bytes::from_static(b"/path/to/invalid\x00character");
    let result = from_shared(input).unwrap_err();

    assert!(matches!(result, ErrorKind::InvalidUriChar));
}

