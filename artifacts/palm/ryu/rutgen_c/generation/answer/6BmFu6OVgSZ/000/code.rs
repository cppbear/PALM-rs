// Answer 0

#[test]
fn test_multiple_of_power_of_5_with_zero_value() {
    assert_eq!(multiple_of_power_of_5(0, 0), true);
    assert_eq!(multiple_of_power_of_5(0, 1), false);
}

#[test]
fn test_multiple_of_power_of_5_with_non_zero_value() {
    assert_eq!(multiple_of_power_of_5(5, 1), true);
    assert_eq!(multiple_of_power_of_5(10, 1), true);
    assert_eq!(multiple_of_power_of_5(25, 2), true);
    assert_eq!(multiple_of_power_of_5(125, 3), true);
    assert_eq!(multiple_of_power_of_5(30, 1), true);
    assert_eq!(multiple_of_power_of_5(30, 2), false);
}

#[test]
fn test_multiple_of_power_of_5_with_large_values() {
    assert_eq!(multiple_of_power_of_5(3125, 5), true);   // 5^5
    assert_eq!(multiple_of_power_of_5(15624, 6), false); // 5^5 is not enough for 6
    assert_eq!(multiple_of_power_of_5(31250, 6), true);  // 5^6
}

#[test]
fn test_multiple_of_power_of_5_boundary_cases() {
    assert_eq!(multiple_of_power_of_5(u64::MAX, 0), true); // Any value should be >= 0
    assert_eq!(multiple_of_power_of_5(u64::MAX, 1), true); // Maximum value should have sufficient factors
}

