// Answer 0

#[test]
fn test_div10() {
    assert_eq!(div10(0), 0); // Test with zero
    assert_eq!(div10(10), 1); // Test with a multiple of 10
    assert_eq!(div10(20), 2); // Test with another multiple of 10
    assert_eq!(div10(99), 9); // Test with a number just below a multiple of 10
    assert_eq!(div10(100), 10); // Test with a larger multiple of 10
    assert_eq!(div10(1), 0); // Test with a number less than 10
    assert_eq!(div10(15), 1); // Test with a non-multiple of 10
    assert_eq!(div10(1000), 100); // Test with a larger number
    assert_eq!(div10(u64::MAX), u64::MAX / 10); // Test with maximum u64 value
}

