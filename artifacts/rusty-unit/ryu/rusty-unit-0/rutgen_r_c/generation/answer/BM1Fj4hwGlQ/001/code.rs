// Answer 0

#[test]
fn test_multiple_of_power_of_5_32() {
    // Test cases where value is a multiple of 5 raised to the power of p
    assert_eq!(multiple_of_power_of_5_32(0, 0), true); // 0 is a special case, since it has infinite factors of 5
    assert_eq!(multiple_of_power_of_5_32(5, 1), true); // 5^1
    assert_eq!(multiple_of_power_of_5_32(25, 2), true); // 5^2
    assert_eq!(multiple_of_power_of_5_32(125, 3), true); // 5^3
    assert_eq!(multiple_of_power_of_5_32(625, 4), true); // 5^4
    
    // Test cases where value is not a multiple of 5 to ensure it returns false
    assert_eq!(multiple_of_power_of_5_32(1, 1), false); // not a multiple of 5
    assert_eq!(multiple_of_power_of_5_32(2, 1), false); // not a multiple of 5
    assert_eq!(multiple_of_power_of_5_32(3, 1), false); // not a multiple of 5
    assert_eq!(multiple_of_power_of_5_32(4, 1), false); // not a multiple of 5
    assert_eq!(multiple_of_power_of_5_32(14, 1), false); // not a multiple of 5
    
    // Test cases where value is a higher power of 5 but p is higher
    assert_eq!(multiple_of_power_of_5_32(125, 4), false); // 5^3 is insufficient for 5^4
    
    // Edge cases
    assert_eq!(multiple_of_power_of_5_32(0, 1), false); // 0 does not meet any positive power requirement
}

