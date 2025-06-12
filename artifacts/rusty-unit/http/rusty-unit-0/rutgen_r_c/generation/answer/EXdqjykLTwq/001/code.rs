// Answer 0

#[test]
fn test_port_u16_with_valid_uri() {
    let uri: Uri = "http://example.org:80/hello/world".parse().unwrap();
    assert_eq!(uri.port_u16(), Some(80));
}

#[test]
fn test_port_u16_with_uri_no_port() {
    let uri: Uri = "http://example.org/hello/world".parse().unwrap();
    assert_eq!(uri.port_u16(), None);
}

#[test]
fn test_port_u16_with_uri_without_authority() {
    let uri: Uri = "http:///hello/world".parse().unwrap();
    assert_eq!(uri.port_u16(), None);
}

#[test]
#[should_panic(expected = "static str is not valid URI")]
fn test_port_u16_with_invalid_uri() {
    let _uri: Uri = "http://example.org:invalid/hello/world".parse().unwrap(); // Invalid port format
}

