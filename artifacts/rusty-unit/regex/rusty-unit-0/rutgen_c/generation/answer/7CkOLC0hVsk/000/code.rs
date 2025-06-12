// Answer 0

#[test]
fn test_canonical_prop_valid_name() {
    let normalized_name = "Lu"; // Uppercase Letter
    let result = canonical_prop(normalized_name);
    assert_eq!(result, Some("Uppercase_Letter"));
}

#[test]
fn test_canonical_prop_invalid_name() {
    let normalized_name = "Invalid_Property_Name";
    let result = canonical_prop(normalized_name);
    assert_eq!(result, None);
}

#[test]
fn test_canonical_prop_empty_string() {
    let normalized_name = "";
    let result = canonical_prop(normalized_name);
    assert_eq!(result, None);
}

#[test]
fn test_canonical_prop_partial_name() {
    let normalized_name = "L";
    let result = canonical_prop(normalized_name);
    assert_eq!(result, None);
}

#[test]
fn test_canonical_prop_case_insensitive() {
    let normalized_name = "lu"; // lowercase version
    let result = canonical_prop(normalized_name);
    assert_eq!(result, Some("Uppercase_Letter"));
}

