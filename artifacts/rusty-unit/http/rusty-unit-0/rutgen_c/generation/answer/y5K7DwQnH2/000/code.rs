// Answer 0

#[test]
fn test_path_and_query_empty() {
    let path_and_query = PathAndQuery::empty();
    assert_eq!(path_and_query.query, u16::MAX);
    assert_eq!(path_and_query.data.bytes.len(), 0);
}

#[test]
fn test_path_and_query_empty_multiple() {
    let path_and_query1 = PathAndQuery::empty();
    let path_and_query2 = PathAndQuery::empty();
    assert_eq!(path_and_query1, path_and_query2);
}

