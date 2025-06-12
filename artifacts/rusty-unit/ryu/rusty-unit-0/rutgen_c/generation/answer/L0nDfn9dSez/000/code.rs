// Answer 0

#[test]
fn test_multiple_of_power_of_2_positive_case() {
    assert!(multiple_of_power_of_2(16, 4));
    assert!(multiple_of_power_of_2(32, 5));
    assert!(multiple_of_power_of_2(0xFFFF, 16));
}

#[test]
fn test_multiple_of_power_of_2_negative_case() {
    assert!(!multiple_of_power_of_2(18, 4));
    assert!(!multiple_of_power_of_2(20, 5));
    assert!(!multiple_of_power_of_2(0xFFFF, 15));
}

#[test]
#[should_panic]
fn test_multiple_of_power_of_2_zero_value() {
    multiple_of_power_of_2(0, 4);
}

#[test]
#[should_panic]
fn test_multiple_of_power_of_2_invalid_power() {
    multiple_of_power_of_2(16, 64);
}

