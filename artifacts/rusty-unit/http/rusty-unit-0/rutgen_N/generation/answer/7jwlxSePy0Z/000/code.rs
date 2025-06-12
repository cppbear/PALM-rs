// Answer 0

#[derive(Debug)]
struct StatusCode;

#[derive(Debug)]
struct InvalidStatusCode;

impl StatusCode {
    fn from_bytes(s: &[u8]) -> Result<StatusCode, InvalidStatusCode> {
        if s.is_empty() {
            Err(InvalidStatusCode)
        } else {
            Ok(StatusCode)
        }
    }
}

fn from_str(s: &str) -> Result<StatusCode, InvalidStatusCode> {
    StatusCode::from_bytes(s.as_ref())
}

#[test]
fn test_from_str_valid_input() {
    let result = from_str("200");
    assert!(result.is_ok());
}

#[test]
fn test_from_str_empty_input() {
    let result = from_str("");
    assert!(result.is_err());
}

#[test]
fn test_from_str_non_empty_input() {
    let result = from_str("404");
    assert!(result.is_ok());
}

