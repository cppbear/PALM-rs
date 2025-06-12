// Answer 0

#[test]
fn test_host_with_valid_registered_name() {
    let uri: Uri = "http://example.com/path".parse().unwrap();
    let _ = uri.host();
}

#[test]
fn test_host_with_valid_ip_literal() {
    let uri: Uri = "http://[127.0.0.1]/path".parse().unwrap();
    let _ = uri.host();
}

#[test]
fn test_host_with_valid_ipv4() {
    let uri: Uri = "http://192.168.1.1/path".parse().unwrap();
    let _ = uri.host();
}

#[test]
fn test_host_with_empty_authority() {
    let uri: Uri = "/path".parse().unwrap();
    let _ = uri.host();
}

#[test]
fn test_host_with_scheme_and_no_authority() {
    let uri: Uri = "http:///path".parse().unwrap();
    let _ = uri.host();
}

#[test]
fn test_host_with_case_insensitive_host() {
    let uri: Uri = "http://ExAmPlE.cOm/path".parse().unwrap();
    let _ = uri.host();
}

#[test]
fn test_host_with_long_registered_name() {
    let uri: Uri = "http://subdomain.example.com/path".parse().unwrap();
    let _ = uri.host();
}

#[test]
fn test_host_with_valid_short_uri() {
    let uri: Uri = "http://a.com".parse().unwrap();
    let _ = uri.host();
}

