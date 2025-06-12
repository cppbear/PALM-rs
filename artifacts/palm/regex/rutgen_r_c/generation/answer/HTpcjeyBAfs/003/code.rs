// Answer 0

#[test]
fn test_ages_valid_age() {
    let result = ages("V1_1");
    assert!(result.is_ok());
    let age_iter = result.unwrap();
    assert_eq!(age_iter.ages.len(), 1);
}

#[test]
fn test_ages_another_valid_age() {
    let result = ages("V5_0");
    assert!(result.is_ok());
    let age_iter = result.unwrap();
    assert_eq!(age_iter.ages.len(), 10);
}

#[test]
fn test_ages_invalid_age() {
    let result = ages("V99_0");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Error::PropertyValueNotFound);
}

#[test]
fn test_ages_boundary_case() {
    let result = ages("V10_0");
    assert!(result.is_ok());
    let age_iter = result.unwrap();
    assert_eq!(age_iter.ages.len(), 19);
}

