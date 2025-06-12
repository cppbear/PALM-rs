// Answer 0

#[test]
fn test_canonical_prop_valid_name() {
    let result = regex_syntax::canonical_prop("L");
    assert_eq!(result, Some("Letter"));
}

#[test]
fn test_canonical_prop_invalid_name() {
    let result = regex_syntax::canonical_prop("INVALID");
    assert_eq!(result, None);
}

#[test]
fn test_canonical_prop_empty_string() {
    let result = regex_syntax::canonical_prop("");
    assert_eq!(result, None);
}

#[test]
fn test_canonical_prop_boundary_value() {
    let result = regex_syntax::canonical_prop("Z");
    assert_eq!(result, Some("Other"));
}

