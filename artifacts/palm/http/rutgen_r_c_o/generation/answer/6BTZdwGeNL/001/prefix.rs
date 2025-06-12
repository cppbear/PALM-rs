// Answer 0

#[test]
fn test_body_success_with_empty_body() {
    let response = Builder::new().body(()).unwrap();
}

#[test]
fn test_body_success_with_small_body() {
    let body = "Hello, world!";
    let response = Builder::new().body(body).unwrap();
}

#[test]
fn test_body_success_with_large_body() {
    let body = vec![0u8; 1048576]; // 1 MB
    let response = Builder::new().body(body).unwrap();
}

#[test]
fn test_body_success_with_status_200_and_http1() {
    let response = Builder::new()
        .status(200)
        .version(Version::Http1)
        .body("Success")
        .unwrap();
}

#[test]
fn test_body_success_with_status_404_and_http2() {
    let response = Builder::new()
        .status(404)
        .version(Version::Http2)
        .body("Not Found")
        .unwrap();
}

#[test]
fn test_body_success_with_header_name_and_value() {
    let response = Builder::new()
        .header("Content-Type", "text/plain")
        .body("Header Test")
        .unwrap();
}

#[test]
#[should_panic]
fn test_body_failure_with_invalid_header() {
    let response = Builder::new()
        .header("Invalid-Header\r\n", "Some Value") // Invalid header name
        .body("This should panic")
        .unwrap();
}

#[test]
#[should_panic]
fn test_body_failure_with_too_large_body() {
    let body = vec![0u8; 1048577]; // Exceeds 1 MB
    let response = Builder::new().body(body).unwrap();
}

