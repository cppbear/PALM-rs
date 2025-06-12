// Answer 0

#[derive(Debug)]
struct StatusCode;

#[derive(Debug)]
struct InvalidStatusCode;

impl StatusCode {
    fn from_bytes(s: &[u8]) -> Result<StatusCode, InvalidStatusCode> {
        // Sample implementation for testing purposes
        if s.is_empty() {
            Err(InvalidStatusCode)
        } else if s.starts_with(b"HTTP/1.1 ") {
            Ok(StatusCode)
        } else {
            Err(InvalidStatusCode)
        }
    }
}

#[test]
fn test_valid_http_status_code() {
    let input = "HTTP/1.1 200 OK";
    let result = from_str(input);
    assert!(result.is_ok());
}

#[test]
fn test_invalid_http_status_code_empty() {
    let input = "";
    let result = from_str(input);
    assert!(result.is_err());
}

#[test]
fn test_invalid_http_status_code_wrong_prefix() {
    let input = "STATUS 404 Not Found";
    let result = from_str(input);
    assert!(result.is_err());
}

#[test]
fn test_valid_status_code_with_space() {
    let input = "HTTP/1.1 404 Not Found";
    let result = from_str(input);
    assert!(result.is_ok());
}

#[test]
fn test_invalid_status_code_with_special_characters() {
    let input = "HTTP/1.1 2@0 OK";
    let result = from_str(input);
    assert!(result.is_err());
}

