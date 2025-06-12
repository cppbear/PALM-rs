// Answer 0

#[test]
fn test_path_and_query_star() {
    struct TestPathAndQuery;

    let path_and_query = TestPathAndQuery::star();
    assert_eq!(path_and_query.data.bytes.as_ref(), b"*");
    assert_eq!(path_and_query.query, NONE);
}

#[test]
fn test_path_and_query_star_query_non_none() {
    struct TestPathAndQuery;

    let path_and_query = TestPathAndQuery::star();
    assert_eq!(path_and_query.query, NONE);
}

#[test]
fn test_path_and_query_star_data_content() {
    struct TestPathAndQuery;

    let path_and_query = TestPathAndQuery::star();
    let expected_data: &[u8] = b"*";
    assert_eq!(path_and_query.data.bytes.as_ref(), expected_data);
}

