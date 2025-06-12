// Answer 0

#[test]
fn test_slash() {
    let path_and_query = PathAndQuery::slash();
    assert_eq!(path_and_query.data.bytes.len(), 1);
    assert_eq!(path_and_query.query, u16::MAX);
    assert_eq!(path_and_query.data.bytes[0], b'/');
}

#[test]
fn test_slash_empty_query() {
    let path_and_query = PathAndQuery::slash();
    assert_eq!(path_and_query.query, u16::MAX);
    assert!(path_and_query.data.bytes.is_ascii());
}

