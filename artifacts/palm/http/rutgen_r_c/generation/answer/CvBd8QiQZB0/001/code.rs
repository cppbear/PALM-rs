// Answer 0

#[test]
fn test_from_str_valid_uri() {
    let valid_uri = "https://example.com/path?query=value";
    let result = Uri::from_str(valid_uri);
    assert!(result.is_ok());
}

#[test]
fn test_from_str_invalid_uri_empty() {
    let empty_uri = "";
    let result = Uri::from_str(empty_uri);
    assert!(result.is_err());
}

#[test]
fn test_from_str_invalid_uri_too_long() {
    let long_uri = "a".repeat(65536); // Exceeds the limit
    let result = Uri::from_str(&long_uri);
    assert!(result.is_err());
}

#[test]
fn test_from_str_missing_scheme() {
    let missing_scheme_uri = "example.com/path?query=value";
    let result = Uri::from_str(missing_scheme_uri);
    assert!(result.is_err());
}

#[test]
fn test_from_str_invalid_characters() {
    let invalid_uri = "https://example.com/path\x00"; // Contains NULL character
    let result = Uri::from_str(invalid_uri);
    assert!(result.is_err());
}

#[test]
fn test_from_str_valid_uri_no_query() {
    let uri_no_query = "http://example.com/path";
    let result = Uri::from_str(uri_no_query);
    assert!(result.is_ok());
}

#[test]
fn test_from_str_valid_uri_with_special_chars() {
    let uri_special_chars = "http://example.com/path?value=1&key=value%20with%20space";
    let result = Uri::from_str(uri_special_chars);
    assert!(result.is_ok());
}

#[test]
fn test_from_str_scheme_case_sensitivity() {
    let uri_lowercase_scheme = "http://example.com";
    let result_lowercase = Uri::from_str(uri_lowercase_scheme);
    assert!(result_lowercase.is_ok());

    let uri_uppercase_scheme = "HTTP://example.com";
    let result_uppercase = Uri::from_str(uri_uppercase_scheme);
    assert!(result_uppercase.is_ok());
}

