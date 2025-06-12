// Answer 0

#[test]
fn test_canonical_script_valid_input() {
    let result = canonical_script("Latin");
    assert_eq!(result, Some("Latn"));
}

#[test]
fn test_canonical_script_empty_string() {
    let result = canonical_script("");
    assert_eq!(result, None);
}

#[test]
fn test_canonical_script_invalid_input() {
    let result = canonical_script("InvalidScript");
    assert_eq!(result, None);
}

#[test]
#[should_panic]
fn test_canonical_script_panic() {
    // This case assumes that the underlying property_values function will panic if "Script" is not available
    // Since we cannot control the global state of property_values, we just invoke it directly.
    canonical_script("Cyrillic"); // Assume this would cause panic with missing "Script"
}

