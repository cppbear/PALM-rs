// Answer 0

#[test]
fn test_class_bytes_range_new_valid_range() {
    let range = ClassBytesRange::new(10, 20);
    assert_eq!(range.start(), 10);
    assert_eq!(range.end(), 20);
}

#[test]
fn test_class_bytes_range_new_reverse_order() {
    let range = ClassBytesRange::new(20, 10);
    assert_eq!(range.start(), 10);
    assert_eq!(range.end(), 20);
}

#[test]
fn test_class_bytes_range_new_equal_bounds() {
    let range = ClassBytesRange::new(15, 15);
    assert_eq!(range.start(), 15);
    assert_eq!(range.end(), 15);
}

#[test]
fn test_class_bytes_range_new_min_bounds() {
    let range = ClassBytesRange::new(u8::MIN, u8::MIN);
    assert_eq!(range.start(), u8::MIN);
    assert_eq!(range.end(), u8::MIN);
}

#[test]
fn test_class_bytes_range_new_max_bounds() {
    let range = ClassBytesRange::new(u8::MAX, u8::MAX);
    assert_eq!(range.start(), u8::MAX);
    assert_eq!(range.end(), u8::MAX);
}

