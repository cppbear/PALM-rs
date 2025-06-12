// Answer 0

#[test]
fn test_from_str_valid_string() {
    let valid_str = "valid_header_value";
    let result = HeaderValue::from_str(valid_str);
    assert!(result.is_ok());
    let header_value = result.unwrap();
    assert_eq!(header_value.to_str().unwrap(), valid_str);
}

#[test]
fn test_from_str_empty_string() {
    let empty_str = "";
    let result = HeaderValue::from_str(empty_str);
    assert!(result.is_ok());
    let header_value = result.unwrap();
    assert_eq!(header_value.len(), 0);
    assert!(header_value.is_empty());
}

#[test]
fn test_from_str_invalid_string() {
    let invalid_str = "\x00invalid_header_value"; // control character
    let result = HeaderValue::from_str(invalid_str);
    assert!(result.is_err());
}

#[test]
fn test_from_str_large_string() {
    let large_str = "a".repeat(100); // large valid string
    let result = HeaderValue::from_str(&large_str);
    assert!(result.is_ok());
    let header_value = result.unwrap();
    assert_eq!(header_value.len(), large_str.len());
}

