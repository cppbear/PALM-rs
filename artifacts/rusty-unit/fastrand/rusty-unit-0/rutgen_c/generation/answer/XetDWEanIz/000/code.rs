// Answer 0

#[test]
fn test_mul_high_u64_basic() {
    assert_eq!(mul_high_u64(1, 1), 0);
    assert_eq!(mul_high_u64(2, 2), 1);
    assert_eq!(mul_high_u64(4, 4), 2);
}

#[test]
fn test_mul_high_u64_large_numbers() {
    assert_eq!(mul_high_u64(1 << 32, 1 << 32), 1);
    assert_eq!(mul_high_u64(1 << 63, 1 << 63), 0);
    assert_eq!(mul_high_u64(u64::MAX, u64::MAX), u64::MAX);
}

#[test]
fn test_mul_high_u64_zero() {
    assert_eq!(mul_high_u64(0, 0), 0);
    assert_eq!(mul_high_u64(0, 1), 0);
    assert_eq!(mul_high_u64(1, 0), 0);
}

#[test]
fn test_mul_high_u64_edge_cases() {
    assert_eq!(mul_high_u64(1, u64::MAX), 0);
    assert_eq!(mul_high_u64(u64::MAX, 1), 0);
    assert_eq!(mul_high_u64(u64::MAX, u64::MAX), u64::MAX);
}

