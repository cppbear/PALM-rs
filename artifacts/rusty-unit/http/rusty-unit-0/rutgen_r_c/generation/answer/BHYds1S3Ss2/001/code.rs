// Answer 0

#[test]
fn test_is_informational_with_100() {
    let status_code = StatusCode::from_u16(100).unwrap();
    assert!(status_code.is_informational());
}

#[test]
fn test_is_informational_with_199() {
    let status_code = StatusCode::from_u16(199).unwrap();
    assert!(status_code.is_informational());
}

#[test]
fn test_is_informational_with_200() {
    let status_code = StatusCode::from_u16(200).unwrap();
    assert!(!status_code.is_informational());
}

#[test]
fn test_is_informational_with_99() {
    let status_code = StatusCode::from_u16(99).unwrap();
    assert!(!status_code.is_informational());
}

#[test]
fn test_is_informational_with_non_informational() {
    let status_code = StatusCode::from_u16(404).unwrap();
    assert!(!status_code.is_informational());
}

#[test]
fn test_is_informational_with_150() {
    let status_code = StatusCode::from_u16(150).unwrap();
    assert!(status_code.is_informational());
}

#[test]
fn test_is_informational_lower_bound() {
    // Panic test case: create a StatusCode with an invalid value
    // This needs to be handled properly in the method, so we assert on panic
    let result = std::panic::catch_unwind(|| {
        StatusCode::from_u16(0);
    });
    assert!(result.is_err());
}

#[test]
fn test_is_informational_upper_bound() {
    // Panic test case: trying to create StatusCode with a value above the maximum value
    let result = std::panic::catch_unwind(|| {
        StatusCode::from_u16(700);
    });
    assert!(result.is_err());
}

