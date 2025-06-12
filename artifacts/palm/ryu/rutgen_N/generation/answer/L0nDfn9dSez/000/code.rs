// Answer 0

#[test]
fn test_multiple_of_power_of_2() {
    assert!(multiple_of_power_of_2(8, 3)); // 8 is 2^3, should return true
    assert!(multiple_of_power_of_2(16, 4)); // 16 is 2^4, should return true
    assert!(multiple_of_power_of_2(0xFFFFFFFFFFFFFFF0, 4)); // All bits 0 except the last 4, should return true
    assert!(!multiple_of_power_of_2(10, 1)); // 10 is not a multiple of 2^1, should return false
}

#[should_panic]
#[test]
fn test_multiple_of_power_of_2_zero_value() {
    multiple_of_power_of_2(0, 2); // Should panic due to assert(value != 0)
}

#[should_panic]
#[test]
fn test_multiple_of_power_of_2_out_of_bounds_power() {
    multiple_of_power_of_2(8, 64); // Should panic due to assert(p < 64)
}

