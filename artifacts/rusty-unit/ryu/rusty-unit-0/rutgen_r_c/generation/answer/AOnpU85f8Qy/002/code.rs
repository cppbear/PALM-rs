// Answer 0

#[test]
fn test_mul_shift_32_valid_input() {
    let m: u32 = 2;
    let factor: u64 = 0x1FFFFFFF; // Large factor
    let shift: i32 = 33; // Valid shift
    let result = mul_shift_32(m, factor, shift);
    assert_eq!(result, 1);
}

#[test]
#[should_panic]
fn test_mul_shift_32_invalid_shift() {
    let m: u32 = 1;
    let factor: u64 = 1;
    let shift: i32 = 32; // Invalid shift, should panic
    mul_shift_32(m, factor, shift);
}

#[test]
#[should_panic]
fn test_mul_shift_32_exceeding_shifted_sum() {
    let m: u32 = u32::max_value(); // Maximum m
    let factor: u64 = u32::max_value() as u64 + 1; // Factor that leads to overflow
    let shift: i32 = 64; // Shift resulting in an exceeding shifted sum
    mul_shift_32(m, factor, shift);
}

