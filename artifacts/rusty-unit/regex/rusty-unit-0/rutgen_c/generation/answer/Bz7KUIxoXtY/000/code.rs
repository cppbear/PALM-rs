// Answer 0

#[test]
fn test_class_bytes_ranges() {
    struct DummyInterval;

    let range1 = ClassBytesRange { start: 1, end: 5 };
    let range2 = ClassBytesRange { start: 6, end: 10 };
    
    let interval_set = IntervalSet::new(vec![range1, range2]);
    let class_bytes = ClassBytes { set: interval_set };

    let ranges = class_bytes.ranges();

    assert_eq!(ranges.len(), 2);
    assert_eq!(ranges[0], ClassBytesRange { start: 1, end: 5 });
    assert_eq!(ranges[1], ClassBytesRange { start: 6, end: 10 });
}

#[test]
fn test_class_bytes_empty_ranges() {
    let empty_class_bytes = ClassBytes::empty();
    let ranges = empty_class_bytes.ranges();

    assert_eq!(ranges.len(), 0);
}

