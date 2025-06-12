// Answer 0

#[test]
fn test_mul_high_u32_small_numbers() {
    let result = mul_high_u32(2, 3);
    assert_eq!(result, 0);
}

#[test]
fn test_mul_high_u32_large_numbers() {
    let result = mul_high_u32(1_000_000, 1_000_000);
    assert_eq!(result, 0);
}

#[test]
fn test_mul_high_u32_max_values() {
    let result = mul_high_u32(u32::MAX, u32::MAX);
    assert_eq!(result, 0xFFFFFFFF);
}

#[test]
fn test_mul_high_u32_edge_case() {
    let result = mul_high_u32(0, 0);
    assert_eq!(result, 0);
}

#[test]
#[should_panic]
fn test_mul_high_u32_overflow() {
    let _ = mul_high_u32(u32::MAX, 2);
}

