// Answer 0

#[test]
fn test_valid_path_and_query() {
    let v = PathAndQuery::from_static("/hello?world");
    assert_eq!(v.path(), "/hello");
    assert_eq!(v.query(), Some("world"));
}

#[test]
#[should_panic]
fn test_invalid_path_missing_query() {
    PathAndQuery::from_static("/hello?");
}

#[test]
#[should_panic]
fn test_invalid_path_special_chars() {
    PathAndQuery::from_static("/hello<>world");
}

#[test]
#[should_panic]
fn test_invalid_path_no_slash() {
    PathAndQuery::from_static("hello?world");
}

#[test]
#[should_panic]
fn test_invalid_path_only_query() {
    PathAndQuery::from_static("?world");
}

#[test]
fn test_valid_path_without_query() {
    let v = PathAndQuery::from_static("/hello");
    assert_eq!(v.path(), "/hello");
    assert_eq!(v.query(), None);
}

#[test]
fn test_valid_long_path_and_query() {
    let v = PathAndQuery::from_static("/a/very/long/path/that/consists/of/many/segments?with=query&parameters");
    assert_eq!(v.path(), "/a/very/long/path/that/consists/of/many/segments");
    assert_eq!(v.query(), Some("with=query&parameters"));
}

