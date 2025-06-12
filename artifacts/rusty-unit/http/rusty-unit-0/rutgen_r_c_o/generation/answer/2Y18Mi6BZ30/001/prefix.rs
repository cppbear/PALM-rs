// Answer 0

#[test]
fn test_post_valid_http_uri() {
    Request::post("http://example.com");
}

#[test]
fn test_post_valid_https_uri() {
    Request::post("https://example.com");
}

#[test]
fn test_post_valid_ftp_uri() {
    Request::post("ftp://example.com");
}

#[test]
fn test_post_valid_mailto_uri() {
    Request::post("mailto:test@example.com");
}

#[test]
fn test_post_valid_data_uri() {
    Request::post("data:text/plain;base64,SGVsbG8sIFdvcmxkIQ==");
}

#[test]
fn test_post_valid_file_uri() {
    Request::post("file:///path/to/file");
}

#[test]
fn test_post_valid_ws_uri() {
    Request::post("ws://example.com/socket");
}

#[test]
fn test_post_valid_wss_uri() {
    Request::post("wss://example.com/socket");
}

#[test]
#[should_panic]
fn test_post_empty_uri() {
    Request::post("");
}

#[test]
#[should_panic]
fn test_post_invalid_uri() {
    Request::post("invalid_uri");
}

#[test]
#[should_panic]
fn test_post_invalid_scheme_uri() {
    Request::post("://");
}

#[test]
#[should_panic]
fn test_post_http_with_no_authority() {
    Request::post("http://:8080");
}

#[test]
#[should_panic]
fn test_post_http_with_invalid_port() {
    Request::post("http://example.com:invalid_port");
}

