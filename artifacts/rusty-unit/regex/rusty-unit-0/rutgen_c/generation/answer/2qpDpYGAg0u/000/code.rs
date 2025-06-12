// Answer 0

#[test]
fn test_lower() {
    let range = ClassBytesRange { start: 10, end: 20 };
    assert_eq!(range.lower(), 10);
}

#[test]
fn test_lower_zero() {
    let range = ClassBytesRange { start: 0, end: 255 };
    assert_eq!(range.lower(), 0);
}

#[test]
fn test_lower_equal_end() {
    let range = ClassBytesRange { start: 50, end: 50 };
    assert_eq!(range.lower(), 50);
}

#[test]
fn test_lower_negative_start() {
    let range = ClassBytesRange { start: u8::MIN, end: u8::MAX };
    assert_eq!(range.lower(), u8::MIN);
}

