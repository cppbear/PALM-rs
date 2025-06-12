// Answer 0

#[test]
fn test_upper() {
    let interval = ClassBytesRange { start: 5, end: 10 };
    assert_eq!(interval.upper(), 10);
}

#[test]
fn test_upper_with_different_values() {
    let interval = ClassBytesRange { start: 0, end: 255 };
    assert_eq!(interval.upper(), 255);
}

#[test]
fn test_upper_with_min_max() {
    let interval_min = ClassBytesRange { start: 0, end: 0 };
    assert_eq!(interval_min.upper(), 0);

    let interval_max = ClassBytesRange { start: u8::MAX, end: u8::MAX };
    assert_eq!(interval_max.upper(), u8::MAX);
}

