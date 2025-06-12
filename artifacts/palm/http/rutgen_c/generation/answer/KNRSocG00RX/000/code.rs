// Answer 0

#[test]
fn test_path_with_non_empty_path() {
    let path_and_query = PathAndQuery::from_static("/hello/world");
    assert_eq!(path_and_query.path(), "/hello/world");
}

#[test]
fn test_path_with_empty_path() {
    let path_and_query = PathAndQuery::empty();
    assert_eq!(path_and_query.path(), "/");
}

#[test]
fn test_path_with_query() {
    let path_and_query = PathAndQuery::from_shared(Bytes::from_static("/path/to/resource?key=value")).unwrap();
    assert_eq!(path_and_query.path(), "/path/to/resource");
}

#[test]
fn test_path_with_star() {
    let path_and_query = PathAndQuery::star();
    assert_eq!(path_and_query.path(), "*");
}

#[test]
fn test_path_with_slash() {
    let path_and_query = PathAndQuery::slash();
    assert_eq!(path_and_query.path(), "/");
}

