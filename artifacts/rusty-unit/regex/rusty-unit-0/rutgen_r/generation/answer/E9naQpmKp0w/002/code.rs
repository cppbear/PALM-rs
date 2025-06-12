// Answer 0

#[test]
fn test_canonical_gencat_assigned() {
    let result = canonical_gencat("assigned");
    assert_eq!(result, Some("Assigned"));
}

#[test]
fn test_canonical_gencat_none() {
    let result = canonical_gencat("unrecognized_value");
    assert_eq!(result, None);
}

