// Answer 0

#[test]
fn test_from_shared_with_valid_path() {
    let input = Bytes::from_static(b"/valid/path?query=value");
    let result = PathAndQuery::from_shared(input).unwrap();
    assert_eq!(result.query, 13);
}

#[test]
fn test_from_shared_with_no_query() {
    let input = Bytes::from_static(b"/path/with/no/query");
    let result = PathAndQuery::from_shared(input).unwrap();
    assert_eq!(result.query, u16::MAX);
}

#[test]
fn test_from_shared_with_fragment() {
    let input = Bytes::from_static(b"/path#fragment");
    let result = PathAndQuery::from_shared(input).unwrap();
    assert_eq!(result.query, u16::MAX);
}

#[test]
fn test_from_shared_with_invalid_character() {
    let input = Bytes::from_static(b"/invalid/path\0");
    let result = PathAndQuery::from_shared(input);
    assert!(result.is_err());
}

#[test]
fn test_from_shared_with_utf8_check() {
    let input = Bytes::from_static(b"/valid/path\xFF");
    let result = PathAndQuery::from_shared(input);
    assert!(result.is_err());
}

#[test]
fn test_from_shared_with_empty_path() {
    let input = Bytes::from_static(b"");
    let result = PathAndQuery::from_shared(input);
    assert!(result.is_err());
}

