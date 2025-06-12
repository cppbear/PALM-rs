// Answer 0

#[test]
fn test_h1_zero() {
    let hash: u64 = 0;
    let result = h1(hash);
    assert_eq!(result, 0);
}

#[test]
fn test_h1_small_positive() {
    let hash: u64 = 10;
    let result = h1(hash);
    assert_eq!(result, 10);
}

#[test]
fn test_h1_small_negative() {
    let hash: u64 = u64::MAX - 10; // Checking behavior for large values
    let result = h1(hash);
    assert_eq!(result, (u64::MAX - 10) as usize);
}

#[test]
fn test_h1_large_value() {
    let hash: u64 = u64::MAX; // Most significant bit set
    let result = h1(hash);
    assert_eq!(result, u64::MAX as usize);
}

#[test]
fn test_h1_mid_range() {
    let hash: u64 = 123456789; // Mid-range value
    let result = h1(hash);
    assert_eq!(result, 123456789);
}

