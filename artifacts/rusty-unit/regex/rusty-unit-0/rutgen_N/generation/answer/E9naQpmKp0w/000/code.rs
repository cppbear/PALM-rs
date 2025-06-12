// Answer 0

#[test]
fn test_canonical_gencat_any() {
    let result = canonical_gencat("any");
    assert_eq!(result, Some("Any"));
}

#[test]
fn test_canonical_gencat_assigned() {
    let result = canonical_gencat("assigned");
    assert_eq!(result, Some("Assigned"));
}

#[test]
fn test_canonical_gencat_ascii() {
    let result = canonical_gencat("ascii");
    assert_eq!(result, Some("ASCII"));
}

#[test]
fn test_canonical_gencat_unknown() {
    let result = canonical_gencat("unknown");
    assert!(result.is_none());
}

