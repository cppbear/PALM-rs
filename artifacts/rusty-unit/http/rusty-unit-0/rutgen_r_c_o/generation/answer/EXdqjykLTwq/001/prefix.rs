// Answer 0

#[test]
fn test_port_u16_valid_port() {
    let uri: Uri = "http://example.org:80/hello/world".parse().unwrap();
    uri.port_u16();
}

#[test]
fn test_port_u16_another_valid_port() {
    let uri: Uri = "https://example.org:443/hello/world".parse().unwrap();
    uri.port_u16();
}

#[test]
fn test_port_u16_min_valid_port() {
    let uri: Uri = "ftp://example.org:0/path".parse().unwrap();
    uri.port_u16();
}

#[test]
fn test_port_u16_max_valid_port() {
    let uri: Uri = "http://example.org:65535/hello/world".parse().unwrap();
    uri.port_u16();
}

#[test]
fn test_port_u16_no_port() {
    let uri: Uri = "http://example.org/hello/world".parse().unwrap();
    uri.port_u16();
}

#[test]
fn test_port_u16_invalid_uri() {
    let uri: Uri = "htp://example.org:80/hello/world".parse().unwrap();
    uri.port_u16();
}

