// Answer 0

#[test]
fn test_mul_high_u64_small_values() {
    assert_eq!(mul_high_u64(1, 1), 0);
    assert_eq!(mul_high_u64(1, 2), 0);
    assert_eq!(mul_high_u64(2, 3), 0);
}

#[test]
fn test_mul_high_u64_large_values() {
    assert_eq!(mul_high_u64(1 << 32, 1 << 32), 1);
    assert_eq!(mul_high_u64(1 << 63, 1 << 1), 1);
    assert_eq!(mul_high_u64(0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF), 0xFFFFFFFFFFFFFFFF);
}

#[test]
fn test_mul_high_u64_boundary_values() {
    assert_eq!(mul_high_u64(0, 0), 0);
    assert_eq!(mul_high_u64(0, u64::MAX), 0);
    assert_eq!(mul_high_u64(u64::MAX, 0), 0);
    assert_eq!(mul_high_u64(u64::MAX, u64::MAX), 0xFFFF_FFFF_FFFF_FFFF);
}

