// Answer 0

#[test]
fn test_empty_class_bytes() {
    let class_bytes = ClassBytes::empty();
    assert_eq!(class_bytes.ranges(), &[]);
}

#[test]
fn test_class_bytes_new_with_one_range() {
    let range = ClassBytesRange { start: 0u8, end: 5u8 };
    let class_bytes = ClassBytes::new(vec![range]);
    assert_eq!(class_bytes.ranges(), &[range]);
}

#[test]
fn test_class_bytes_new_with_multiple_ranges() {
    let range1 = ClassBytesRange { start: 0u8, end: 5u8 };
    let range2 = ClassBytesRange { start: 10u8, end: 15u8 };
    let class_bytes = ClassBytes::new(vec![range1, range2]);
    assert_eq!(class_bytes.ranges(), &[range1, range2]);
}

