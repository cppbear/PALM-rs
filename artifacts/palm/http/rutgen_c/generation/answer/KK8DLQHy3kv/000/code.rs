// Answer 0

#[test]
fn test_try_from_valid_ascii() {
    let input = "valid-header";
    let result = HeaderName::try_from(input);
    assert!(result.is_ok());
}

#[test]
fn test_try_from_valid_header_chars() {
    let input = "X-Custom-Header";
    let result = HeaderName::try_from(input);
    assert!(result.is_ok());
}

#[test]
fn test_try_from_invalid_header_chars() {
    let input = "invalid\x00header";
    let result = HeaderName::try_from(input);
    assert!(result.is_err());
}

#[test]
fn test_try_from_empty_string() {
    let input = "";
    let result = HeaderName::try_from(input);
    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_try_from_panic_on_static_str() {
    let input: &'static str = "static-header";
    let _header_name = HeaderName::from_static(input);
}

