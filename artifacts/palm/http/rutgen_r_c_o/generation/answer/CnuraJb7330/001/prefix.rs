// Answer 0

#[test]
fn test_scheme_none_relative_uri() {
    let uri: Uri = "/path/to/resource".parse().unwrap();
    let result = uri.scheme();
}

#[test]
fn test_scheme_none_absolute_uri_no_scheme() {
    let uri: Uri = "username:password@example.com/path".parse().unwrap();
    let result = uri.scheme();
}

#[test]
fn test_scheme_none_empty_uri() {
    let uri: Uri = "".parse().unwrap();
    let result = uri.scheme();
}

#[test]
fn test_scheme_none_uri_with_only_query() {
    let uri: Uri = "?key=value".parse().unwrap();
    let result = uri.scheme();
}

#[test]
fn test_scheme_none_uri_with_fragment() {
    let uri: Uri = "#fragment".parse().unwrap();
    let result = uri.scheme();
}

#[test]
fn test_scheme_none_relative_uri_with_query() {
    let uri: Uri = "/path?key=value".parse().unwrap();
    let result = uri.scheme();
}

