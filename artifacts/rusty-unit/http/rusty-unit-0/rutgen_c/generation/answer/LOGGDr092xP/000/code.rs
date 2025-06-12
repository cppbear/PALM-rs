// Answer 0

#[test]
fn test_from_str_valid_header_name() {
    let result = HeaderName::from_str("Accept");
    assert!(result.is_ok());
    let header_name = result.unwrap();
    assert_eq!(header_name.as_str(), "Accept");
}

#[test]
fn test_from_str_invalid_header_name() {
    let result = HeaderName::from_str("InvalidHeader@Name!");
    assert!(result.is_err());
}

#[test]
fn test_from_str_empty_string() {
    let result = HeaderName::from_str("");
    assert!(result.is_err());
}

#[test]
fn test_from_str_whitespace_string() {
    let result = HeaderName::from_str("   ");
    assert!(result.is_err());
}

#[test]
fn test_from_str_uppercase_header_name() {
    let result = HeaderName::from_str("ACCEPT");
    assert!(result.is_ok());
    let header_name = result.unwrap();
    assert_eq!(header_name.as_str(), "ACCEPT");
}

