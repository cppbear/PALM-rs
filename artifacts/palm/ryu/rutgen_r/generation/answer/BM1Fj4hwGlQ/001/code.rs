// Answer 0

#[test]
fn test_multiple_of_power_of_5_32_zero() {
    assert_eq!(multiple_of_power_of_5_32(0, 0), true);
    assert_eq!(multiple_of_power_of_5_32(0, 1), false);
}

#[test]
fn test_multiple_of_power_of_5_32_one() {
    assert_eq!(multiple_of_power_of_5_32(1, 0), true);
    assert_eq!(multiple_of_power_of_5_32(1, 1), false);
}

#[test]
fn test_multiple_of_power_of_5_32_five() {
    assert_eq!(multiple_of_power_of_5_32(5, 1), true);
    assert_eq!(multiple_of_power_of_5_32(5, 2), false);
}

#[test]
fn test_multiple_of_power_of_5_32_twenty_five() {
    assert_eq!(multiple_of_power_of_5_32(25, 2), true);
    assert_eq!(multiple_of_power_of_5_32(25, 3), false);
}

#[test]
fn test_multiple_of_power_of_5_32_hundred_and_twenty_five() {
    assert_eq!(multiple_of_power_of_5_32(125, 3), true);
    assert_eq!(multiple_of_power_of_5_32(125, 4), false);
}

#[test]
fn test_multiple_of_power_of_5_32_large_values() {
    assert_eq!(multiple_of_power_of_5_32(3125, 5), true); // 5^5
    assert_eq!(multiple_of_power_of_5_32(3125, 6), false); 
}

#[test]
fn test_multiple_of_power_of_5_32_edge_cases() {
    assert_eq!(multiple_of_power_of_5_32(10, 1), true); // 5^1 is a factor of 10
    assert_eq!(multiple_of_power_of_5_32(10, 2), false); // 5^2 is not a factor of 10
}

