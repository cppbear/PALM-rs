// Answer 0

#[test]
fn test_uri_path_absolute() {
    let uri: http::Uri = "http://example.com/path/to/resource".parse().unwrap();
    assert_eq!(uri.path(), "/path/to/resource");
}

#[test]
fn test_uri_path_relative() {
    let uri: http::Uri = "/relative/path".parse().unwrap();
    assert_eq!(uri.path(), "/relative/path");
}

#[test]
fn test_uri_path_empty_string() {
    let uri: http::Uri = "http://example.com/".parse().unwrap();
    assert_eq!(uri.path(), "/");
}

#[test]
fn test_uri_path_with_query() {
    let uri: http::Uri = "http://example.com/path?query=param".parse().unwrap();
    assert_eq!(uri.path(), "/path");
}

#[test]
fn test_uri_path_wildcard() {
    let uri: http::Uri = "*".parse().unwrap();
    assert_eq!(uri.path(), "*");
}

