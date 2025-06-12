// Answer 0

#[test]
fn test_uri_with_valid_uri() {
    let req = Builder::new()
        .uri("http://example.com")
        .body(())
        .unwrap();
}

#[test]
fn test_uri_with_https_uri() {
    let req = Builder::new()
        .uri("https://www.rust-lang.org")
        .body(())
        .unwrap();
}

#[test]
fn test_uri_with_ftp_uri() {
    let req = Builder::new()
        .uri("ftp://ftp.is.co.za/rfc/rfc1808.txt")
        .body(())
        .unwrap();
}

#[test]
#[should_panic]
fn test_uri_with_empty_string() {
    let req = Builder::new()
        .uri("")
        .body(())
        .unwrap();
}

#[test]
#[should_panic]
fn test_uri_with_invalid_uri() {
    let req = Builder::new()
        .uri("invalid-uri")
        .body(())
        .unwrap();
}

#[test]
#[should_panic]
fn test_uri_with_invalid_http_uri() {
    let req = Builder::new()
        .uri("http://-invalid.com")
        .body(())
        .unwrap();
}

#[test]
#[should_panic]
fn test_uri_with_incomplete_ipv6() {
    let req = Builder::new()
        .uri("http://[::1")
        .body(())
        .unwrap();
}

#[test]
#[should_panic]
fn test_uri_with_missing_scheme() {
    let req = Builder::new()
        .uri("://missing-scheme.com")
        .body(())
        .unwrap();
}

#[test]
fn test_uri_with_edge_case_path_query_fragment() {
    let req = Builder::new()
        .uri("http://example.com/path?query=value#fragment")
        .body(())
        .unwrap();
}

#[test]
fn test_uri_with_ip_address() {
    let req = Builder::new()
        .uri("http://123.45.67.89:8080")
        .body(())
        .unwrap();
}

#[test]
fn test_uri_with_port_and_path() {
    let req = Builder::new()
        .uri("http://example.com:80/path")
        .body(())
        .unwrap();
}

#[test]
fn test_uri_with_special_characters() {
    let req = Builder::new()
        .uri("http://example.com/special_chars?with=%20&symbols=!@#$%^&*()")
        .body(())
        .unwrap();
}

