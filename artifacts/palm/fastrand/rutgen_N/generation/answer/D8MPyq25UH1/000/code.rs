// Answer 0

#[test]
fn test_mul_high_u32_standard() {
    assert_eq!(mul_high_u32(0, 0), 0);
    assert_eq!(mul_high_u32(1, 1), 0);
    assert_eq!(mul_high_u32(2, 2), 0);
    assert_eq!(mul_high_u32(65536, 65536), 1);
    assert_eq!(mul_high_u32(4294967295, 4294967295), 0);
}

#[test]
fn test_mul_high_u32_boundary() {
    assert_eq!(mul_high_u32(4294967295, 1), 0);
    assert_eq!(mul_high_u32(1, 4294967295), 0);
    assert_eq!(mul_high_u32(32768, 32768), 0);
}

#[test]
fn test_mul_high_u32_large_values() {
    assert_eq!(mul_high_u32(65535, 65535), 0);
    assert_eq!(mul_high_u32(40000, 40000), 0);
}

