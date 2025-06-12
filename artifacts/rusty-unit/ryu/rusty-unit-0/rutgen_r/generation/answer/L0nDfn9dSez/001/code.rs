// Answer 0

#[test]
fn test_multiple_of_power_of_2_true() {
    assert!(multiple_of_power_of_2(8, 3)); // 8 is multiple of 2^3 (8)
    assert!(multiple_of_power_of_2(16, 4)); // 16 is multiple of 2^4 (16)
    assert!(multiple_of_power_of_2(0b111000, 3)); // 56 is multiple of 2^3 (8)
}

#[test]
fn test_multiple_of_power_of_2_false() {
    assert!(!multiple_of_power_of_2(10, 3)); // 10 is not a multiple of 2^3 (8)
    assert!(!multiple_of_power_of_2(20, 4)); // 20 is not a multiple of 2^4 (16)
}

#[test]
#[should_panic]
fn test_multiple_of_power_of_2_zero_value() {
    multiple_of_power_of_2(0, 0); // should panic because value is 0
}

#[test]
#[should_panic]
fn test_multiple_of_power_of_2_out_of_bounds_p() {
    multiple_of_power_of_2(1, 64); // should panic because p is not less than 64
}

