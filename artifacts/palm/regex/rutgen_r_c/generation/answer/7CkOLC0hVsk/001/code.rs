// Answer 0

#[test]
fn test_canonical_prop_valid_name() {
    let name = "General_Category";
    let result = canonical_prop(name);
    assert_eq!(result, Some("General_Category"));
}

#[test]
fn test_canonical_prop_invalid_name() {
    let name = "Invalid_Property";
    let result = canonical_prop(name);
    assert_eq!(result, None);
}

#[test]
fn test_canonical_prop_empty_string() {
    let name = "";
    let result = canonical_prop(name);
    assert_eq!(result, None);
}

#[test]
fn test_canonical_prop_whitespace() {
    let name = "   ";
    let result = canonical_prop(name);
    assert_eq!(result, None);
}

#[test]
fn test_canonical_prop_special_characters() {
    let name = "@#$%^&*()";
    let result = canonical_prop(name);
    assert_eq!(result, None);
}

#[test]
fn test_canonical_prop_mixed_case_name() {
    let name = "general_category";
    let result = canonical_prop(name);
    assert_eq!(result, Some("General_Category"));
}

#[test]
#[should_panic]
fn test_canonical_prop_non_utf8_bytes() {
    let name = std::str::from_utf8(&[0xFF, 0xFF, 0xFF]).unwrap();
    canonical_prop(name);
}

