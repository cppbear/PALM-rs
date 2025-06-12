// Answer 0

#[test]
#[should_panic] // This test is expected to panic because shifted_sum exceeds u32::max_value()
fn test_mul_shift_32_panic_condition() {
    let m: u32 = 1; // minimal valid input
    let factor: u64 = (u64::MAX >> 1) + 1; // making factor large enough to trigger the panic condition
    let shift: i32 = 33; // satisfies the constraint shift > 32

    let _result = mul_shift_32(m, factor, shift); // calling the function, should panic
}

#[test]
fn test_mul_shift_32_maximum_shift() {
    let m: u32 = u32::MAX; // maximal valid input
    let factor: u64 = 1; // small factor to validate behavior under standard conditions
    let shift: i32 = 33; // satisfies the constraint shift > 32

    let result = mul_shift_32(m, factor, shift);
    assert_eq!(result, 0); // the expected return value should be 0
}

#[test]
fn test_mul_shift_32_large_inputs() {
    let m: u32 = 1; // minimal valid input
    let factor: u64 = u64::MAX; // using maximum allowed factor
    let shift: i32 = 34; // satisfies the constraint shift > 32

    let result = mul_shift_32(m, factor, shift);
    assert!(result <= u32::max_value()); // check result is within bounds
}

#[test]
fn test_mul_shift_32_edge_case() {
    let m: u32 = 2; // medium valid input
    let factor: u64 = 2; // moderate factor
    let shift: i32 = 33; // satisfies the constraint shift > 32

    let result = mul_shift_32(m, factor, shift);
    assert_eq!(result, 1); // expected output check
}

