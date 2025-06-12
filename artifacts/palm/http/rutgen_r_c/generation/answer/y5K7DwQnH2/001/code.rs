// Answer 0

#[test]
fn test_empty_path_and_query() {
    let result = PathAndQuery::empty();
    assert_eq!(result.query, u16::MAX);
    let expected_data = ByteStr::new();
    assert_eq!(result.data, expected_data);
}

#[test]
fn test_empty_path_and_query_data() {
    let result = PathAndQuery::empty();
    assert!(result.data.bytes.is_empty());
}

