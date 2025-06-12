// Answer 0

#[test]
fn test_multiple_of_power_of_2_32_true() {
    assert!(multiple_of_power_of_2_32(0b00001111, 4));
    assert!(multiple_of_power_of_2_32(0, 4));
    assert!(multiple_of_power_of_2_32(0b11110000, 4));
}

#[test]
fn test_multiple_of_power_of_2_32_false() {
    assert!(!multiple_of_power_of_2_32(0b00001111, 3));
    assert!(!multiple_of_power_of_2_32(0b11110001, 4));
}

#[test]
fn test_multiple_of_power_of_2_32_boundary() {
    assert!(multiple_of_power_of_2_32(0b1, 1));
    assert!(!multiple_of_power_of_2_32(0b1, 2));
}

