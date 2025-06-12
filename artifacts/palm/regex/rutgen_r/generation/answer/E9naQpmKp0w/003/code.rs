// Answer 0

#[test]
fn test_canonical_gencat_ascii() {
    let result = canonical_gencat("ascii");
    assert_eq!(result, Some("ASCII"));
}

#[test]
fn test_canonical_gencat_non_standard() {
    let result = canonical_gencat("not_a_category");
    assert!(result.is_none());
}

