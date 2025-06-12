// Answer 0

#[derive(Debug)]
struct HeaderValue {
    value: String,
}

impl HeaderValue {
    fn from_str(s: &str) -> Result<Self, &'static str> {
        if s.is_empty() {
            Err("Empty string is not a valid header value")
        } else {
            Ok(HeaderValue { value: s.to_string() })
        }
    }
}

#[test]
fn test_header_value_from_str_valid() {
    let result = HeaderValue::from_str("valid-header-value");
    assert!(result.is_ok());
    let header_value = result.unwrap();
    assert_eq!(header_value.value, "valid-header-value");
}

#[test]
fn test_header_value_from_str_empty() {
    let result = HeaderValue::from_str("");
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), "Empty string is not a valid header value");
}

