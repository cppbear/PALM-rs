// Answer 0

#[test]
fn test_mul_high_u64_small_numbers() {
    let result = mul_high_u64(1, 2);
    assert_eq!(result, 0);
}

#[test]
fn test_mul_high_u64_numbers_that_cause_no_overflow() {
    let result = mul_high_u64(1000, 1000);
    assert_eq!(result, 0);
}

#[test]
fn test_mul_high_u64_large_numbers() {
    let result = mul_high_u64(1 << 32, 1 << 32);
    assert_eq!(result, 1 << 32);
}

#[test]
fn test_mul_high_u64_max_values() {
    let result = mul_high_u64(u64::MAX, u64::MAX);
    assert_eq!(result, 0); // (u64::MAX * u64::MAX) >> 64 indeed wraps around
}

#[test]
fn test_mul_high_u64_with_zero() {
    let result = mul_high_u64(0, 0);
    assert_eq!(result, 0);
}

#[test]
fn test_mul_high_u64_one_side_zero() {
    let result = mul_high_u64(0, 10);
    assert_eq!(result, 0);

    let result = mul_high_u64(10, 0);
    assert_eq!(result, 0);
}

