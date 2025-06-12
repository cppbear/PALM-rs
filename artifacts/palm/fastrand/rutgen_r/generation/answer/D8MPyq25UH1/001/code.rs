// Answer 0

#[test]
fn test_mul_high_u32_small_values() {
    assert_eq!(mul_high_u32(2, 3), 0);
    assert_eq!(mul_high_u32(1, 1), 0);
}

#[test]
fn test_mul_high_u32_boundary_values() {
    assert_eq!(mul_high_u32(u32::MAX, 1), (u32::MAX as u64 >> 32) as u32);
    assert_eq!(mul_high_u32(1, u32::MAX), (u32::MAX as u64 >> 32) as u32);
}

#[test]
fn test_mul_high_u32_large_values() {
    assert_eq!(mul_high_u32(65536, 65536), 1);
    assert_eq!(mul_high_u32(4294967295, 4294967295), (u64::MAX >> 32) as u32);
}

