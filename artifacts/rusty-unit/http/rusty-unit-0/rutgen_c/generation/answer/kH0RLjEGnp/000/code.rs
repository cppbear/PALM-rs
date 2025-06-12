// Answer 0

#[test]
fn test_host_with_valid_uri() {
    let uri: Uri = Uri::from_static("http://example.org:80/hello/world");
    assert_eq!(uri.host(), Some("example.org"));
}

#[test]
fn test_host_with_relative_uri() {
    let uri: Uri = Uri::from_static("/hello/world");
    assert!(uri.host().is_none());
}

#[test]
fn test_host_with_ip_literal() {
    let uri: Uri = Uri::from_static("http://[192.168.1.1]:80/path");
    assert_eq!(uri.host(), Some("192.168.1.1"));
}

#[test]
fn test_host_with_ipv4_address() {
    let uri: Uri = Uri::from_static("http://192.168.1.1:80/path");
    assert_eq!(uri.host(), Some("192.168.1.1"));
}

#[test]
fn test_host_with_registered_name() {
    let uri: Uri = Uri::from_static("http://example.com:123/path");
    assert_eq!(uri.host(), Some("example.com"));
}

#[test]
#[should_panic]
fn test_host_with_invalid_uri() {
    let uri: Uri = Uri::from_static("http://invalid_uri");
    let _ = uri.host(); // Expect panic on invalid URI
}

