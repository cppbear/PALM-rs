// Answer 0

#[test]
fn test_div5() {
    // Test case for a typical positive number
    assert_eq!(div5(10), 2);
    
    // Test case for another typical positive number
    assert_eq!(div5(25), 5);
    
    // Test case for a larger positive number
    assert_eq!(div5(100), 20);
    
    // Test case for zero, which should be valid
    assert_eq!(div5(0), 0);
    
    // Test case for a number less than 5 where it should return 0
    assert_eq!(div5(3), 0);
    
    // Test case for a number exactly 5
    assert_eq!(div5(5), 1);
    
    // Test case for the maximum value of u64 to check upper boundaries
    assert_eq!(div5(u64::MAX), u64::MAX / 5);
}

