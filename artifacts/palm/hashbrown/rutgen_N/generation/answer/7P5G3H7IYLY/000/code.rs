// Answer 0

#[test]
fn test_h1_lower_bound() {
    let hash = 0u64;
    let result = h1(hash);
    assert_eq!(result, 0);
}

#[test]
fn test_h1_middle_value() {
    let hash = 1234567890u64;
    let result = h1(hash);
    assert_eq!(result, 1234567890);
}

#[test]
fn test_h1_upper_bound() {
    let hash = u64::MAX;
    let result = h1(hash);
    assert_eq!(result, u64::MAX as usize);
}

