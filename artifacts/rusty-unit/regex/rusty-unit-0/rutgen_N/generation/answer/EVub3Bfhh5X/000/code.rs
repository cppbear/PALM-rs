// Answer 0

#[test]
fn test_property_values_valid_property() {
    let property_name = "Lu"; // Uppercase letters
    let result = regex_syntax::property_values(property_name);
    assert!(result.is_some());
}

#[test]
fn test_property_values_invalid_property() {
    let property_name = "InvalidProperty"; // Non-existing property
    let result = regex_syntax::property_values(property_name);
    assert!(result.is_none());
}

#[test]
fn test_property_values_empty_property() {
    let property_name = ""; // Empty string
    let result = regex_syntax::property_values(property_name);
    assert!(result.is_none());
}

#[test]
fn test_property_values_whitespace_property() {
    let property_name = " "; // Whitespace string
    let result = regex_syntax::property_values(property_name);
    assert!(result.is_none());
}

