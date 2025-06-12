// Answer 0

#[test]
fn test_as_str_with_query() {
    let path_and_query = PathAndQuery::from_static("/hello/world?key=value&foo=bar");
    assert_eq!(path_and_query.as_str(), "/hello/world?key=value&foo=bar");
}

#[test]
fn test_as_str_without_query() {
    let path_and_query = PathAndQuery::from_static("/hello/world");
    assert_eq!(path_and_query.as_str(), "/hello/world");
}

#[test]
fn test_as_str_empty_path() {
    let path_and_query = PathAndQuery::empty();
    assert_eq!(path_and_query.as_str(), "/");
}

#[test]
fn test_as_str_slash() {
    let path_and_query = PathAndQuery::slash();
    assert_eq!(path_and_query.as_str(), "/");
}

#[test]
fn test_as_str_star() {
    let path_and_query = PathAndQuery::star();
    assert_eq!(path_and_query.as_str(), "*");
}

