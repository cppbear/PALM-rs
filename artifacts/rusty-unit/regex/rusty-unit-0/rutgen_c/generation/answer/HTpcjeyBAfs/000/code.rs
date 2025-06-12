// Answer 0

#[test]
fn test_ages_known_version() {
    let result = ages("V2_1");
    assert!(result.is_ok());
    let age_iter = result.unwrap();
    assert_eq!(age_iter.ages.len(), 3); // V1_1, V2_0, V2_1
}

#[test]
fn test_ages_boundary_version() {
    let result = ages("V1_1");
    assert!(result.is_ok());
    let age_iter = result.unwrap();
    assert_eq!(age_iter.ages.len(), 1); // Only V1_1
}

#[test]
fn test_ages_unknown_version() {
    let result = ages("V99_9");
    assert!(result.is_err());
    assert_eq!(result.err(), Some(Error::PropertyValueNotFound));
}

