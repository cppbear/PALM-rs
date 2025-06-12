// Answer 0

#[test]
fn test_mul_shift_32_basic() {
    let m: u32 = 2;
    let factor: u64 = 3;
    let shift: i32 = 33; // shift > 32

    let result = mul_shift_32(m, factor, shift);
    assert_eq!(result, 1); // (2 * 3) >> 1 = 6 >> 1 = 3
}

#[test]
fn test_mul_shift_32_max_value() {
    let m: u32 = u32::MAX;
    let factor: u64 = 1;
    let shift: i32 = 33; // shift > 32

    let result = mul_shift_32(m, factor, shift);
    assert_eq!(result, u32::MAX); // (u32::MAX * 1) >> 1
}

#[test]
#[should_panic]
fn test_mul_shift_32_invalid_shift() {
    let m: u32 = 2;
    let factor: u64 = 3;
    let shift: i32 = 32; // shift is not greater than 32

    let _ = mul_shift_32(m, factor, shift); // Should panic on debug assertion
}

#[test]
fn test_mul_shift_32_large_values() {
    let m: u32 = 0xFFFF_FFFF;
    let factor: u64 = 0xFFFF_FFFF_FFFF_FFFF;
    let shift: i32 = 34; // shift > 32

    let result = mul_shift_32(m, factor, shift);
    assert_eq!(result, 0xFFFFFFFE); // (u32::MAX * u64::MAX) >> 2
}

