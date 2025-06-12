// Answer 0

#[test]
fn test_uri_with_port() {
    let uri: Uri = "http://example.org:80/hello/world".parse().unwrap();
    let port = uri.port().unwrap();
    assert_eq!(port.port, 80);
}

#[test]
fn test_uri_without_port() {
    let uri: Uri = "http://example.org/hello/world".parse().unwrap();
    assert!(uri.port().is_none());
}

#[test]
fn test_relative_uri_without_port() {
    let uri: Uri = "/hello/world".parse().unwrap();
    assert!(uri.port().is_none());
}

#[test]
fn test_uri_with_another_port() {
    let uri: Uri = "http://example.com:443/secure".parse().unwrap();
    let port = uri.port().unwrap();
    assert_eq!(port.port, 443);
}

