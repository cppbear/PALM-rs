// Answer 0

#[test]
fn test_push_single_range() {
    let mut class_bytes = ClassBytes::new(vec![]);
    let range = ClassBytesRange { start: 10, end: 20 };
    class_bytes.push(range);
    let ranges = class_bytes.ranges();
    assert_eq!(ranges.len(), 1);
    assert_eq!(ranges[0], ClassBytesRange { start: 10, end: 20 });
}

#[test]
fn test_push_multiple_ranges() {
    let mut class_bytes = ClassBytes::new(vec![]);
    class_bytes.push(ClassBytesRange { start: 1, end: 5 });
    class_bytes.push(ClassBytesRange { start: 6, end: 10 });
    let ranges = class_bytes.ranges();
    assert_eq!(ranges.len(), 2);
    assert_eq!(ranges[0], ClassBytesRange { start: 1, end: 5 });
    assert_eq!(ranges[1], ClassBytesRange { start: 6, end: 10 });
}

#[test]
fn test_push_overlapping_ranges() {
    let mut class_bytes = ClassBytes::new(vec![]);
    class_bytes.push(ClassBytesRange { start: 5, end: 15 });
    class_bytes.push(ClassBytesRange { start: 10, end: 20 });
    let ranges = class_bytes.ranges();
    assert_eq!(ranges.len(), 2); // Assuming the implementation does not merge overlapping ranges
    assert_eq!(ranges[0], ClassBytesRange { start: 5, end: 15 });
    assert_eq!(ranges[1], ClassBytesRange { start: 10, end: 20 });
}

#[test]
fn test_push_empty_range() {
    let mut class_bytes = ClassBytes::new(vec![]);
    class_bytes.push(ClassBytesRange { start: 5, end: 5 }); // Empty range
    let ranges = class_bytes.ranges();
    assert_eq!(ranges.len(), 1);
    assert_eq!(ranges[0], ClassBytesRange { start: 5, end: 5 });
}

