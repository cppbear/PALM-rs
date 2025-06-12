// Answer 0

#[test]
fn test_uri_with_port() {
    use http::Uri;

    let uri: Uri = "http://example.org:80/hello/world".parse().unwrap();
    let port = uri.port().unwrap();
    assert_eq!(port.as_u16(), 80);
}

#[test]
fn test_uri_without_port() {
    use http::Uri;

    let uri: Uri = "http://example.org/hello/world".parse().unwrap();
    assert!(uri.port().is_none());
}

#[test]
fn test_relative_uri_without_port() {
    use http::Uri;

    let uri: Uri = "/hello/world".parse().unwrap();
    assert!(uri.port().is_none());
}

