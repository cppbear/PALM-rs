// Answer 0

#[test]
fn test_multiple_of_power_of_5_32_with_zero() {
    assert_eq!(multiple_of_power_of_5_32(0, 1), false);
}

#[test]
fn test_multiple_of_power_of_5_32_with_non_multiple() {
    assert_eq!(multiple_of_power_of_5_32(3, 1), false);
}

#[test]
fn test_multiple_of_power_of_5_32_with_one() {
    assert_eq!(multiple_of_power_of_5_32(1, 0), true);
}

#[test]
fn test_multiple_of_power_of_5_32_with_five() {
    assert_eq!(multiple_of_power_of_5_32(5, 1), true);
}

#[test]
fn test_multiple_of_power_of_5_32_with_twenty_five() {
    assert_eq!(multiple_of_power_of_5_32(25, 2), true);
}

#[test]
fn test_multiple_of_power_of_5_32_with_one_hundred() {
    assert_eq!(multiple_of_power_of_5_32(100, 2), true);
}

#[test]
fn test_multiple_of_power_of_5_32_with_large_value() {
    assert_eq!(multiple_of_power_of_5_32(125, 3), true);
}

#[test]
fn test_multiple_of_power_of_5_32_with_large_non_multiple() {
    assert_eq!(multiple_of_power_of_5_32(124, 3), false);
}

