// Answer 0

#[test]
fn test_div10() {
    assert_eq!(div10(10), 1);
    assert_eq!(div10(20), 2);
    assert_eq!(div10(0), 0);
    assert_eq!(div10(99), 9);
    assert_eq!(div10(100), 10);
    assert_eq!(div10(1234567890), 123456789);
}

