// Answer 0

#[test]
#[should_panic]
fn test_mul_shift_32_panic_shift() {
    let m: u32 = 1;
    let factor: u64 = 2;
    let shift: i32 = 32; // This should not trigger panic since shift must be > 32
    let _ = mul_shift_32(m, factor, shift);
}

#[test]
#[should_panic]
fn test_mul_shift_32_panic_shifted_sum() {
    let m: u32 = 4294967295; // Max value
    let factor: u64 = 18446744073709551615; // Max value
    let shift: i32 = 64; // Should trigger panic since the shifted_sum will exceed u32::max_value()
    let _ = mul_shift_32(m, factor, shift);
}

#[test]
fn test_mul_shift_32_valid_range() {
    let m: u32 = 123456; // Arbitrary value
    let factor: u64 = 123456789; // Arbitrary value
    let shift: i32 = 40; // Valid shift
    let _ = mul_shift_32(m, factor, shift);
}

#[test]
fn test_mul_shift_32_boundary_high() {
    let m: u32 = 4294967295; // Max value
    let factor: u64 = 1; // Small factor
    let shift: i32 = 33; // Just above the minimum boundary
    let _ = mul_shift_32(m, factor, shift);
}

#[test]
fn test_mul_shift_32_boundary_low() {
    let m: u32 = 0; // Min value
    let factor: u64 = 1; // Small factor
    let shift: i32 = 34; // Valid shift
    let _ = mul_shift_32(m, factor, shift);
}

#[test]
fn test_mul_shift_32_factor_zero() {
    let m: u32 = 345678; // Arbitrary value
    let factor: u64 = 0; // Zero factor
    let shift: i32 = 35; // Valid shift
    let _ = mul_shift_32(m, factor, shift);
}

