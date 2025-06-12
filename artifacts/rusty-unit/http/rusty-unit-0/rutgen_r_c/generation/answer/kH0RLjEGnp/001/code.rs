// Answer 0

#[test]
fn test_host_with_valid_absolute_uri() {
    let uri: Uri = Uri::from_static("http://example.com:80/path/data");
    assert_eq!(uri.host(), Some("example.com"));
}

#[test]
fn test_host_with_valid_absolute_uri_with_ipv4() {
    let uri: Uri = Uri::from_static("http://192.168.1.1:8080/path/data");
    assert_eq!(uri.host(), Some("192.168.1.1"));
}

#[test]
fn test_host_with_valid_absolute_uri_with_ipv6() {
    let uri: Uri = Uri::from_static("http://[::1]:80/path/data");
    assert_eq!(uri.host(), Some("[::1]"));
}

#[test]
fn test_host_with_valid_absolute_uri_with_case_insensitive_host() {
    let uri: Uri = Uri::from_static("http://EXAMPLE.COM:80/path/data");
    assert_eq!(uri.host(), Some("EXAMPLE.COM"));
}

#[test]
fn test_host_with_relative_uri() {
    let uri: Uri = Uri::from_static("/hello/world");
    assert!(uri.host().is_none());
}

#[test]
fn test_host_with_uri_without_authority() {
    let uri: Uri = Uri::from_static("http:///path/data");
    assert!(uri.host().is_none());
}

#[test]
fn test_host_with_empty_authority() {
    let uri: Uri = Uri::from_static("http://:80/path/data");
    assert!(uri.host().is_none());
}

