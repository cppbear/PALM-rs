// Answer 0

#[test]
fn test_mul_high_u64_small_numbers() {
    let a = 1;
    let b = 1;
    mul_high_u64(a, b);
}

#[test]
fn test_mul_high_u64_large_numbers() {
    let a = u64::MAX;
    let b = u64::MAX;
    mul_high_u64(a, b);
}

#[test]
fn test_mul_high_u64_zero() {
    let a = 0;
    let b = u64::MAX;
    mul_high_u64(a, b);
}

#[test]
fn test_mul_high_u64_mid_range() {
    let a = 1000000000;
    let b = 2000000000;
    mul_high_u64(a, b);
}

#[test]
fn test_mul_high_u64_boundary_condition() {
    let a = u64::MAX - 1;
    let b = 2;
    mul_high_u64(a, b);
}

#[test]
fn test_mul_high_u64_mid_range_case_two() {
    let a = 123456789;
    let b = 987654321;
    mul_high_u64(a, b);
}

