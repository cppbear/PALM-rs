// Answer 0

#[test]
fn test_mul_shift_32_valid() {
    let m: u32 = 1;
    let factor: u64 = u32::max_value() as u64 * 2; // Using a large factor while ensuring proper calculation
    let shift: i32 = 40; // Ensuring shift is greater than 32

    let result = mul_shift_32(m, factor, shift);
    assert_eq!(result, u32::max_value()); // Expecting the maximum u32 value
}

#[test]
#[should_panic(expected = "assertion failed: shift > 32")]
fn test_mul_shift_32_panic_shift() {
    let m: u32 = 1;
    let factor: u64 = 1;
    let shift: i32 = 32; // This should trigger panic

    mul_shift_32(m, factor, shift);
}

#[test]
#[should_panic(expected = "assertion failed: shifted_sum <= u32::max_value() as u64")]
fn test_mul_shift_32_panic_shifted_sum() {
    let m: u32 = u32::max_value(); // Large m
    let factor: u64 = u32::max_value() as u64 * 2; // Large factor to ensure shifted sum exceeds max
    let shift: i32 = 40; // Ensuring shift is greater than 32

    mul_shift_32(m, factor, shift); // This should trigger panic
}

