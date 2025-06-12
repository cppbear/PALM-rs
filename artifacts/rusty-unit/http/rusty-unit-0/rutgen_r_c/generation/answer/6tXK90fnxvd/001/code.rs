// Answer 0

#[test]
fn test_try_from_valid_string() {
    let input: &[u8] = b"/path/to/resource";
    let result = PathAndQuery::try_from(input);
    assert!(result.is_ok());
    let path_and_query = result.unwrap();
    assert_eq!(path_and_query.data.bytes.as_ref(), input);
    assert_eq!(path_and_query.query, u16::MAX);
}

#[test]
fn test_try_from_invalid_char() {
    let input: &[u8] = b"/path/to/resouce\xFF";
    let result = PathAndQuery::try_from(input);
    assert!(result.is_err());
}

#[test]
fn test_try_from_with_query() {
    let input: &[u8] = b"/path/to/resource?query=1";
    let result = PathAndQuery::try_from(input);
    assert!(result.is_ok());
    let path_and_query = result.unwrap();
    assert_eq!(path_and_query.data.bytes.as_ref(), b"/path/to/resource");
    assert_eq!(path_and_query.query, 15); // query position should be at index 15
}

#[test]
fn test_try_from_with_fragment() {
    let input: &[u8] = b"/path/to/resource#fragment";
    let result = PathAndQuery::try_from(input);
    assert!(result.is_ok());
    let path_and_query = result.unwrap();
    assert_eq!(path_and_query.data.bytes.as_ref(), b"/path/to/resource");
    assert_eq!(path_and_query.query, u16::MAX);
}

#[test]
fn test_try_from_empty_input() {
    let input: &[u8] = b"";
    let result = PathAndQuery::try_from(input);
    assert!(result.is_ok());
    let path_and_query = result.unwrap();
    assert_eq!(path_and_query.data.bytes.as_ref(), b"");
    assert_eq!(path_and_query.query, u16::MAX);
}

#[test]
fn test_try_from_maybe_not_utf8() {
    let input: &[u8] = b"/path\xFF/resource";
    let result = PathAndQuery::try_from(input);
    assert!(result.is_err());
}

