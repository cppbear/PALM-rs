// Answer 0

#[test]
fn test_class_bytes_range_new_valid_range() {
    let range = ClassBytesRange::new(10, 20);
    assert_eq!(range.start(), 10);
    assert_eq!(range.end(), 20);
}

#[test]
fn test_class_bytes_range_new_equal_start_end() {
    let range = ClassBytesRange::new(15, 15);
    assert_eq!(range.start(), 15);
    assert_eq!(range.end(), 15);
}

#[test]
fn test_class_bytes_range_new_reverse_order() {
    let range = ClassBytesRange::new(20, 10);
    assert_eq!(range.start(), 10);
    assert_eq!(range.end(), 20);
}

#[test]
fn test_class_bytes_range_new_min_boundary() {
    let range = ClassBytesRange::new(0, 0);
    assert_eq!(range.start(), 0);
    assert_eq!(range.end(), 0);
}

#[test]
fn test_class_bytes_range_new_max_boundary() {
    let range = ClassBytesRange::new(255, 255);
    assert_eq!(range.start(), 255);
    assert_eq!(range.end(), 255);
}

#[test]
fn test_class_bytes_range_new_exceeding_max_boundary() {
    let range = ClassBytesRange::new(256, 255);
    assert_eq!(range.start(), 255);
    assert_eq!(range.end(), 256);
}

#[test]
fn test_class_bytes_range_new_invalid_range() {
    let range = ClassBytesRange::new(255, 256);
    assert_eq!(range.start(), 255);
    assert_eq!(range.end(), 256);
}

