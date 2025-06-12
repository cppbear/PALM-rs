// Answer 0

#[test]
fn test_uri_with_valid_port_0() {
    let uri: Uri = "http://example.com:0/hello/world".parse().unwrap();
    let port = uri.port();
}

#[test]
fn test_uri_with_valid_port_65535() {
    let uri: Uri = "http://example.com:65535/hello/world".parse().unwrap();
    let port = uri.port();
}

#[test]
fn test_uri_with_missing_port() {
    let uri: Uri = "http://example.com/hello/world".parse().unwrap();
    let port = uri.port();
}

#[test]
fn test_uri_without_authority() {
    let uri: Uri = "/hello/world".parse().unwrap();
    let port = uri.port();
}

#[test]
fn test_uri_with_port_inside_path() {
    let uri: Uri = "http://example.com/path/to/resource:80".parse().unwrap();
    let port = uri.port();
}

#[test]
fn test_uri_with_auth_and_port() {
    let uri: Uri = "http://username:password@example.com:123/path/data".parse().unwrap();
    let port = uri.port();
}

#[test]
fn test_uri_with_http_scheme_no_port() {
    let uri: Uri = "http://example.org/hello/world".parse().unwrap();
    let port = uri.port();
}

#[test]
fn test_uri_with_https_scheme_with_port() {
    let uri: Uri = "https://example.org:443/hello/world".parse().unwrap();
    let port = uri.port();
}

#[test]
fn test_uri_with_invalid_scheme() {
    let uri: Uri = "invalidscheme://example.org:123/hello/world".parse().unwrap();
    let port = uri.port();
}

#[test]
fn test_uri_with_non_numeric_port() {
    let uri: Uri = "http://example.com:abc/hello/world".parse().unwrap();
    let port = uri.port();
}

