// Answer 0

#[test]
fn test_multiple_of_power_of_5() {
    // Test case where value is a multiple of 5^p
    assert_eq!(multiple_of_power_of_5(25, 2), true); // 25 = 5^2
    assert_eq!(multiple_of_power_of_5(125, 3), true); // 125 = 5^3

    // Test case where value is less than 5^p
    assert_eq!(multiple_of_power_of_5(4, 1), false); // 4 < 5^1
    assert_eq!(multiple_of_power_of_5(24, 2), false); // 24 < 5^2

    // Test case where value is exactly 5^p
    assert_eq!(multiple_of_power_of_5(5, 1), true); // 5 = 5^1
    assert_eq!(multiple_of_power_of_5(1, 0), true); // 1 = 5^0

    // Test case where value has no factors of 5
    assert_eq!(multiple_of_power_of_5(6, 1), false); // 6 has 0 factors of 5
    assert_eq!(multiple_of_power_of_5(11, 2), false); // 11 has 0 factors of 5

    // Edge case: Testing with very large numbers
    assert_eq!(multiple_of_power_of_5(3125, 5), true); // 3125 = 5^5
    assert_eq!(multiple_of_power_of_5(15624, 4), false); // 15624 has 0 factors of 5
}

