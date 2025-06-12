// Answer 0

#[test]
fn test_multiple_of_power_of_5_32() {
    assert_eq!(multiple_of_power_of_5_32(25, 2), true); // 25 = 5^2
    assert_eq!(multiple_of_power_of_5_32(5, 1), true);  // 5 = 5^1
    assert_eq!(multiple_of_power_of_5_32(1, 0), true);  // 1 = 5^0
    assert_eq!(multiple_of_power_of_5_32(100, 2), true); // 100 = 5^2 * 4
    assert_eq!(multiple_of_power_of_5_32(30, 1), true);  // 30 = 5^1 * 6
    assert_eq!(multiple_of_power_of_5_32(8, 1), false);  // 8 = not multiple of 5
    assert_eq!(multiple_of_power_of_5_32(12, 1), false); // 12 = not multiple of 5
    assert_eq!(multiple_of_power_of_5_32(0, 1), false);  // 0 does not count as multiple of any power of 5
}

#[test]
fn test_multiple_of_power_of_5_32_edge_cases() {
    assert_eq!(multiple_of_power_of_5_32(0, 0), true);  // 0 vs 0 (should be true)
    assert_eq!(multiple_of_power_of_5_32(125, 3), true); // 125 = 5^3
    assert_eq!(multiple_of_power_of_5_32(125, 4), false); // 125 = 5^3 not 5^4
    assert_eq!(multiple_of_power_of_5_32(1, 1), false);  // 1 vs 1 (not a multiple) 
}

