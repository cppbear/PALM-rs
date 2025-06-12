// Answer 0

#[test]
fn test_port_with_absolute_uri_with_port() {
    use http::Uri;
    let uri: Uri = "http://example.org:80/hello/world".parse().unwrap();
    let port = uri.port().unwrap();
    assert_eq!(port.as_u16(), 80);
}

#[test]
fn test_port_with_absolute_uri_without_port() {
    use http::Uri;
    let uri: Uri = "http://example.org/hello/world".parse().unwrap();
    assert!(uri.port().is_none());
}

#[test]
fn test_port_with_relative_uri() {
    use http::Uri;
    let uri: Uri = "/hello/world".parse().unwrap();
    assert!(uri.port().is_none());
}

#[test]
fn test_port_with_ipv4_address() {
    use http::Uri;
    let uri: Uri = "http://192.168.1.1:8080/test".parse().unwrap();
    let port = uri.port().unwrap();
    assert_eq!(port.as_u16(), 8080);
}

#[test]
fn test_port_with_ipv6_address() {
    use http::Uri;
    let uri: Uri = "http://[2001:db8::1]:3000/test".parse().unwrap();
    let port = uri.port().unwrap();
    assert_eq!(port.as_u16(), 3000);
}

#[test]
fn test_port_with_default_http_port() {
    use http::Uri;
    let uri: Uri = "http://example.com:80".parse().unwrap();
    let port = uri.port().unwrap();
    assert_eq!(port.as_u16(), 80);
}

#[test]
fn test_port_with_default_https_port() {
    use http::Uri;
    let uri: Uri = "https://example.com:443".parse().unwrap();
    let port = uri.port().unwrap();
    assert_eq!(port.as_u16(), 443);
}

