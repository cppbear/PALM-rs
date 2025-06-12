// Answer 0

#[test]
fn test_canonical_script_valid() {
    let result = canonical_script("Latin");
    assert_eq!(result, Some("Latin"));
}

#[test]
fn test_canonical_script_invalid() {
    let result = canonical_script("InvalidScript");
    assert_eq!(result, None);
}

#[test]
fn test_canonical_script_empty() {
    let result = canonical_script("");
    assert_eq!(result, None);
}

#[test]
fn test_canonical_script_whitespace() {
    let result = canonical_script("   ");
    assert_eq!(result, None);
}

