// Answer 0

#[test]
fn test_multiple_of_power_of_2_32() {
    // Test case where value is a multiple of 2^p
    assert_eq!(multiple_of_power_of_2_32(0b00000000_00000000_00000000_00000000, 1), true); // 0 is multiple of any power of 2
    assert_eq!(multiple_of_power_of_2_32(0b00000000_00000000_00000000_00000000, 5), true);
    assert_eq!(multiple_of_power_of_2_32(0b00000000_00000000_00000000_00000000, 31), true);
    
    assert_eq!(multiple_of_power_of_2_32(0b00000000_00000000_00000000_00001000, 3), true); // 8 is multiple of 2^3
    assert_eq!(multiple_of_power_of_2_32(0b00000000_00000000_00000000_00011100, 2), true); // 28 is multiple of 2^2
    
    // Test case where value is NOT a multiple of 2^p
    assert_eq!(multiple_of_power_of_2_32(0b00000000_00000000_00000000_00000001, 1), false); // 1 is not multiple of 2^1
    assert_eq!(multiple_of_power_of_2_32(0b00000000_00000000_00000000_00000010, 2), false); // 2 is not multiple of 2^2
    assert_eq!(multiple_of_power_of_2_32(0b00000000_00000000_00000000_00001111, 4), false); // 15 is not multiple of 2^4
    
    // Edge case: maximum values for p
    assert_eq!(multiple_of_power_of_2_32(0b11111111_11111111_11111111_11111111, 31), false); // max u32 not a multiple of 2^31
    assert_eq!(multiple_of_power_of_2_32(0b11111111_11111111_11111111_11111111, 30), false); // max u32 not a multiple of 2^30
    assert_eq!(multiple_of_power_of_2_32(0b00000000_00000000_00000000_00000000, 32), true); // edge case p = 32 returns true since 0 is multiple of 2^n for any n
}

