// Answer 0

#[test]
fn test_from_static_http_valid_uri() {
    let uri = Uri::from_static("http://example.com/foo");
}

#[test]
fn test_from_static_https_valid_uri() {
    let uri = Uri::from_static("https://valid.uri/path");
}

#[test]
fn test_from_static_ftp_valid_uri() {
    let uri = Uri::from_static("ftp://user:pass@host:21/path");
}

#[test]
fn test_from_static_localhost() {
    let uri = Uri::from_static("http://localhost");
}

#[test]
fn test_from_static_ipv4_address() {
    let uri = Uri::from_static("http://127.0.0.1");
}

#[test]
fn test_from_static_with_query() {
    let uri = Uri::from_static("http://example.com/query?param=value");
}

#[test]
fn test_from_static_empty_path() {
    let uri = Uri::from_static("http://example.com/");
}

#[test]
fn test_from_static_no_scheme() {
    let uri = Uri::from_static("example.com/foo");
}

#[test]
#[should_panic]
fn test_from_static_invalid_uri_empty() {
    let uri = Uri::from_static("");
}

#[test]
#[should_panic]
fn test_from_static_invalid_uri_too_long() {
    let long_uri = "http://" + &"a".repeat(65535) + ".com";
    let uri = Uri::from_static(&long_uri);
}

#[test]
#[should_panic]
fn test_from_static_invalid_uri_missing_host() {
    let uri = Uri::from_static("http:///foo");
}

