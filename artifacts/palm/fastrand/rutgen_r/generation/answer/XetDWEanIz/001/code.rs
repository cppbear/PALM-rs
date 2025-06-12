// Answer 0

#[test]
fn test_mul_high_u64_basic() {
    assert_eq!(mul_high_u64(1, 1), 0);
    assert_eq!(mul_high_u64(1, 2), 0);
    assert_eq!(mul_high_u64(2, 2), 0);
}

#[test]
fn test_mul_high_u64_large_values() {
    assert_eq!(mul_high_u64(1 << 32, 1 << 32), 1);
    assert_eq!(mul_high_u64(1 << 40, 1 << 40), 256);
    assert_eq!(mul_high_u64(1 << 63, 1 << 63), 9223372036854775808);
}

#[test]
fn test_mul_high_u64_max_values() {
    assert_eq!(mul_high_u64(u64::MAX, 1), 0);
    assert_eq!(mul_high_u64(u64::MAX, 2), 1);
    assert_eq!(mul_high_u64(u64::MAX, u64::MAX), 1152921504606846976);
} 

#[test]
fn test_mul_high_u64_edge_cases() {
    assert_eq!(mul_high_u64(0, 0), 0);
    assert_eq!(mul_high_u64(0, 1), 0);
    assert_eq!(mul_high_u64(1, 0), 0);
}

