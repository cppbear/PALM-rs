// Answer 0

#[test]
fn test_decimal_length9() {
    // Test input that is exactly 10, which should return 2 and not panic
    let result = decimal_length9(10);
    assert_eq!(result, 2);
}

#[test]
fn test_decimal_length9_boundary_cases() {
    // Testing boundary cases just below each threshold
    assert_eq!(decimal_length9(9), 1);    // should return 1
    assert_eq!(decimal_length9(99), 2);   // should return 2
    assert_eq!(decimal_length9(999), 3);  // should return 3
    assert_eq!(decimal_length9(9999), 4); // should return 4
    assert_eq!(decimal_length9(99999), 5);// should return 6
    assert_eq!(decimal_length9(999999), 6);// should return 7
    assert_eq!(decimal_length9(9999999), 7);// should return 8
}

