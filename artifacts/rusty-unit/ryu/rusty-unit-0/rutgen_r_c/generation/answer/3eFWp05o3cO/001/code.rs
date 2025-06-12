// Answer 0

#[test]
fn test_multiple_of_power_of_2_32() {
    // Test case where value is 0, should return true for any p
    assert_eq!(multiple_of_power_of_2_32(0, 0), true);
    assert_eq!(multiple_of_power_of_2_32(0, 1), true);
    assert_eq!(multiple_of_power_of_2_32(0, 5), true);
    
    // Test case where value is a power of 2 (e.g., 4), should return true for p <= 2
    assert_eq!(multiple_of_power_of_2_32(4, 0), true);
    assert_eq!(multiple_of_power_of_2_32(4, 2), true);
    assert_eq!(multiple_of_power_of_2_32(4, 1), false);
    
    // Test case where value is a multiple of 2^p
    assert_eq!(multiple_of_power_of_2_32(8, 3), true);  // 8 = 2^3
    assert_eq!(multiple_of_power_of_2_32(16, 4), true); // 16 = 2^4
    assert_eq!(multiple_of_power_of_2_32(32, 5), true); // 32 = 2^5
    
    // Test case where value is not a multiple of 2^p
    assert_eq!(multiple_of_power_of_2_32(8, 2), false);  // 8 is not multiple of 2^2 (4)
    assert_eq!(multiple_of_power_of_2_32(8, 1), false);  // 8 is not multiple of 2^1 (2)
    
    // Boundary Test: maximum value for u32 and p
    assert_eq!(multiple_of_power_of_2_32(u32::MAX, 31), false); // u32::MAX is not multiple of 2^31
    assert_eq!(multiple_of_power_of_2_32(u32::MAX, 30), false); // u32::MAX is not multiple of 2^30
    assert_eq!(multiple_of_power_of_2_32(1, 0), true);  // 1 is a multiple of 2^0 (1)
    
    // Edge case where p is larger than the bits of u32
    // We don't expect panic here; should logically return false.
    assert_eq!(multiple_of_power_of_2_32(1, 32), false); // 1 is not multiple of 2^32
}

