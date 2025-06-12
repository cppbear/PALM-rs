// Answer 0

#[test]
fn test_as_str_with_non_empty_path() {
    let path_and_query = PathAndQuery::from_static("/test/path");
    assert_eq!(path_and_query.as_str(), "/test/path");
}

#[test]
fn test_as_str_with_non_empty_query() {
    let path_and_query = PathAndQuery::from_static("/test/path?key=value");
    assert_eq!(path_and_query.as_str(), "/test/path?key=value");
}

#[test]
fn test_as_str_with_non_ascii_characters() {
    let path_and_query = PathAndQuery::from_static("/teste/paç/área?query=voâ=");
    assert_eq!(path_and_query.as_str(), "/teste/paç/área?query=voâ=");
}

#[test]
fn test_as_str_with_special_characters() {
    let path_and_query = PathAndQuery::from_static("/path/with/special#characters");
    assert_eq!(path_and_query.as_str(), "/path/with/special#characters");
}

#[test]
fn test_as_str_starts_with_slash() {
    let path_and_query = PathAndQuery::from_static("/api/v1/resource");
    assert_eq!(path_and_query.as_str(), "/api/v1/resource");
}

#[test]
fn test_as_str_when_empty() {
    let path_and_query = PathAndQuery::empty();
    assert_eq!(path_and_query.as_str(), "/");
}

