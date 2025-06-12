// Answer 0

#[test]
fn test_property_values_valid_property() {
    let result = property_values("gc"); // Using a valid property name
    assert!(result.is_some());
}

#[test]
fn test_property_values_invalid_property() {
    let result = property_values("invalid_property_name"); // Using an invalid property name
    assert!(result.is_none());
}

#[test]
fn test_property_values_empty_property_name() {
    let result = property_values(""); // Using an empty property name
    assert!(result.is_none());
}

#[test]
fn test_property_values_whitespace_property_name() {
    let result = property_values("   "); // Using a whitespace-only property name
    assert!(result.is_none());
}

#[test]
fn test_property_values_non_ascii_property_name() {
    let result = property_values("Пример"); // Using a non-ASCII property name
    assert!(result.is_none());
}

