// Answer 0

#[test]
fn test_is_nonfinite() {
    // Test case for NaN
    let nan: f32 = f32::from_bits(0x7fc00000); // NaN representation
    assert!(nan.is_nonfinite());

    // Test case for positive infinity
    let pos_inf: f32 = f32::from_bits(0x7f800000); // Positive infinity representation
    assert!(pos_inf.is_nonfinite());

    // Test case for negative infinity
    let neg_inf: f32 = f32::from_bits(0xff800000); // Negative infinity representation
    assert!(neg_inf.is_nonfinite());

    // Test case for a finite number
    let finite_num: f32 = 1.0;
    assert!(!finite_num.is_nonfinite());
}

