// Answer 0

#[test]
fn test_trace_with_valid_http_uri() {
    let uri = "http://valid.url";
    Request::trace(uri);
}

#[test]
fn test_trace_with_valid_https_uri() {
    let uri = "https://valid.url/path?query=value";
    Request::trace(uri);
}

#[test]
fn test_trace_with_valid_localhost_uri() {
    let uri = "http://localhost:8080";
    Request::trace(uri);
}

#[test]
fn test_trace_with_valid_ftp_uri() {
    let uri = "ftp://valid.url";
    Request::trace(uri);
}

#[test]
#[should_panic]
fn test_trace_with_invalid_uri() {
    let uri = "invalid-url";
    Request::trace(uri);
}

#[test]
#[should_panic]
fn test_trace_with_empty_uri() {
    let uri = "";
    Request::trace(uri);
}

