// Answer 0

#[test]
fn test_port_u16_with_valid_uri() {
    let uri: Uri = Uri::from_static("http://example.org:80/hello/world");
    assert_eq!(uri.port_u16(), Some(80));
}

#[test]
fn test_port_u16_with_uri_without_port() {
    let uri: Uri = Uri::from_static("http://example.org/hello/world");
    assert_eq!(uri.port_u16(), None);
}

#[test]
fn test_port_u16_with_uri_with_ssl() {
    let uri: Uri = Uri::from_static("https://example.org:443/hello/world");
    assert_eq!(uri.port_u16(), Some(443));
}

#[test]
fn test_port_u16_with_uri_with_invalid_port() {
    let uri: Uri = Uri::from_static("http://example.org:non_numeric/hello/world");
    assert_eq!(uri.port_u16(), None);
}

