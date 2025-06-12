// Answer 0

#[test]
fn test_port_u16_with_valid_uri() {
    use http::Uri;

    let uri: Uri = "http://example.org:80/hello/world".parse().unwrap();
    assert_eq!(uri.port_u16(), Some(80));
}

#[test]
fn test_port_u16_without_port() {
    use http::Uri;

    let uri: Uri = "http://example.org/hello/world".parse().unwrap();
    assert_eq!(uri.port_u16(), None);
}

#[test]
fn test_port_u16_with_default_port() {
    use http::Uri;

    let uri: Uri = "http://example.org:0/hello/world".parse().unwrap();
    assert_eq!(uri.port_u16(), None);
}

#[test]
fn test_port_u16_with_max_port() {
    use http::Uri;

    let uri: Uri = "http://example.org:65535/hello/world".parse().unwrap();
    assert_eq!(uri.port_u16(), Some(65535));
}

#[test]
#[should_panic]
fn test_port_u16_with_invalid_uri() {
    use http::Uri;

    let _uri: Uri = "http://example.org:invalid_port/hello/world".parse().unwrap();
}

