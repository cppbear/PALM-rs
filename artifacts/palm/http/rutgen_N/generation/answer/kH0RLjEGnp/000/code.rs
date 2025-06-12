// Answer 0

#[test]
fn test_host_absolute_uri() {
    use http::Uri;
    let uri: Uri = "http://example.org:80/hello/world".parse().unwrap();
    assert_eq!(uri.host(), Some("example.org"));
}

#[test]
fn test_host_ip_literal() {
    use http::Uri;
    let uri: Uri = "http://[192.168.1.1]:80/hello/world".parse().unwrap();
    assert_eq!(uri.host(), Some("192.168.1.1"));
}

#[test]
fn test_host_ipv4_address() {
    use http::Uri;
    let uri: Uri = "http://127.0.0.1:8080/test".parse().unwrap();
    assert_eq!(uri.host(), Some("127.0.0.1"));
}

#[test]
fn test_host_registered_name_case_insensitivity() {
    use http::Uri;
    let uri: Uri = "http://Example.org/path".parse().unwrap();
    assert_eq!(uri.host(), Some("Example.org"));
}

#[test]
fn test_host_relative_uri() {
    use http::Uri;
    let uri: Uri = "/hello/world".parse().unwrap();
    assert!(uri.host().is_none());
}

#[test]
fn test_host_no_authority() {
    use http::Uri;
    let uri: Uri = "http:///path".parse().unwrap();
    assert!(uri.host().is_none());
}

