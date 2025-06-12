// Answer 0

#[test]
fn test_from_i128_with_valid_u64() {
    let result = Number::from_i128(256);
    assert!(result.is_some());
    assert_eq!(result.unwrap().as_u64(), Some(256));
}

#[test]
fn test_from_i128_with_valid_i64() {
    let result = Number::from_i128(-128);
    assert!(result.is_some());
    assert_eq!(result.unwrap().as_i64(), Some(-128));
}

#[test]
fn test_from_i128_below_i64_min() {
    let result = Number::from_i128(i128::MIN);
    assert!(result.is_none());
}

#[test]
fn test_from_i128_above_u64_max() {
    let result = Number::from_i128(i128::MAX);
    assert!(result.is_none());
}

