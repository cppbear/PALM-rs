// Answer 0

#[test]
fn test_query_with_query_string_component() {
    let path_and_query = PathAndQuery::from_static("/hello/world?key=value&foo=bar");
    assert_eq!(path_and_query.query(), Some("key=value&foo=bar"));
}

#[test]
fn test_query_without_query_string_component() {
    let path_and_query = PathAndQuery::from_static("/hello/world");
    assert!(path_and_query.query().is_none());
}

#[test]
fn test_query_empty_query_string_component() {
    let path_and_query = PathAndQuery::from_static("/hello/world?");
    assert_eq!(path_and_query.query(), Some(""));
}

#[test]
fn test_query_special_characters_in_query() {
    let path_and_query = PathAndQuery::from_static("/hello/world?key%20value&foo=bar");
    assert_eq!(path_and_query.query(), Some("key%20value&foo=bar"));
}

#[test]
fn test_query_fragment_after_query() {
    let path_and_query = PathAndQuery::from_static("/hello/world?key=value#fragment");
    assert_eq!(path_and_query.query(), Some("key=value"));
}

