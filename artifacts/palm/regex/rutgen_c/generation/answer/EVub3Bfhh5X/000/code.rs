// Answer 0

#[test]
fn test_property_values_valid_property() {
    const CANONICAL_PROPERTY: &str = "General_Category";
    let result = property_values(CANONICAL_PROPERTY);
    assert!(result.is_some());
}

#[test]
fn test_property_values_invalid_property() {
    const INVALID_PROPERTY: &str = "Invalid_Property_Name";
    let result = property_values(INVALID_PROPERTY);
    assert!(result.is_none());
}

#[test]
fn test_property_values_empty_string() {
    const EMPTY_PROPERTY: &str = "";
    let result = property_values(EMPTY_PROPERTY);
    assert!(result.is_none());
}

