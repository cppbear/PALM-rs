// Answer 0

#[derive(Debug)]
struct HeaderValue {
    value: String,
}

impl HeaderValue {
    fn from_str(s: &str) -> Result<Self, &'static str> {
        if s.is_empty() {
            Err("Header value cannot be empty")
        } else {
            Ok(HeaderValue { value: s.to_string() })
        }
    }
}

#[test]
fn test_header_value_from_str_valid() {
    let input = "valid-header-value";
    let result = HeaderValue::from_str(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value, input);
}

#[test]
fn test_header_value_from_str_empty() {
    let input = "";
    let result = HeaderValue::from_str(input);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Header value cannot be empty");
}

#[test]
fn test_header_value_from_str_whitespace() {
    let input = "    ";
    let result = HeaderValue::from_str(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value, input);
}

#[test]
fn test_header_value_from_str_special_chars() {
    let input = "!@#$%^&*()_+";
    let result = HeaderValue::from_str(input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value, input);
}

#[test]
fn test_header_value_from_str_long_string() {
    let input = "a".repeat(1000);
    let result = HeaderValue::from_str(&input);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value, input);
}

