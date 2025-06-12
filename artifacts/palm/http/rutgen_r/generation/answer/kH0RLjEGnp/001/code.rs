// Answer 0

#[test]
fn test_host_absolute_uri() {
    use http::Uri;
    let uri: Uri = "http://example.org:80/hello/world".parse().unwrap();
    assert_eq!(uri.host(), Some("example.org"));
}

#[test]
fn test_host_absolute_uri_with_ipv4() {
    use http::Uri;
    let uri: Uri = "http://192.168.1.1:8080/path".parse().unwrap();
    assert_eq!(uri.host(), Some("192.168.1.1"));
}

#[test]
fn test_host_absolute_uri_with_ipv6() {
    use http::Uri;
    let uri: Uri = "http://[2001:0db8:85a3:0000:0000:8a2e:0370:7334]:80/path".parse().unwrap();
    assert_eq!(uri.host(), Some("2001:0db8:85a3:0000:0000:8a2e:0370:7334"));
}

#[test]
fn test_host_relative_uri() {
    use http::Uri;
    let uri: Uri = "/hello/world".parse().unwrap();
    assert!(uri.host().is_none());
}

#[test]
fn test_host_without_port() {
    use http::Uri;
    let uri: Uri = "http://example.com/hello/world".parse().unwrap();
    assert_eq!(uri.host(), Some("example.com"));
}

#[test]
fn test_host_with_case_insensitivity() {
    use http::Uri;
    let uri: Uri = "http://ExAmPlE.cOm:80/hello".parse().unwrap();
    assert_eq!(uri.host(), Some("ExAmPlE.cOm"));
}

