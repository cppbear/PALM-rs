// Answer 0

#[test]
fn test_decimal_length9() {
    assert_eq!(super::decimal_length9(9), 1);
    assert_eq!(super::decimal_length9(10), 2);
    assert_eq!(super::decimal_length9(99), 2);
    assert_eq!(super::decimal_length9(100), 3);
    assert_eq!(super::decimal_length9(999), 4);
    assert_eq!(super::decimal_length9(1000), 4);
    assert_eq!(super::decimal_length9(9999), 5);
    assert_eq!(super::decimal_length9(10000), 5);
    assert_eq!(super::decimal_length9(99999), 6);
    assert_eq!(super::decimal_length9(100000), 6);
    assert_eq!(super::decimal_length9(999999), 7);
    assert_eq!(super::decimal_length9(1000000), 7);
    assert_eq!(super::decimal_length9(9999999), 8);
    assert_eq!(super::decimal_length9(10000000), 8);
    assert_eq!(super::decimal_length9(999999999), 9);
}

