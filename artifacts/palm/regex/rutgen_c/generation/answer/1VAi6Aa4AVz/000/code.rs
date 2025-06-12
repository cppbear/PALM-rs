// Answer 0

#[test]
fn test_class_bytes_new_with_non_overlapping_ranges() {
    let ranges = vec![
        ClassBytesRange { start: 1, end: 3 },
        ClassBytesRange { start: 4, end: 6 },
    ];
    let class_bytes = ClassBytes::new(ranges);
    assert_eq!(class_bytes.ranges().len(), 2);
    assert_eq!(class_bytes.ranges()[0], ClassBytesRange { start: 1, end: 3 });
    assert_eq!(class_bytes.ranges()[1], ClassBytesRange { start: 4, end: 6 });
}

#[test]
fn test_class_bytes_new_with_overlapping_ranges() {
    let ranges = vec![
        ClassBytesRange { start: 1, end: 5 },
        ClassBytesRange { start: 3, end: 7 },
    ];
    let class_bytes = ClassBytes::new(ranges);
    assert_eq!(class_bytes.ranges().len(), 1); // Assuming ranges merge
    assert_eq!(class_bytes.ranges()[0], ClassBytesRange { start: 1, end: 7 });
}

#[test]
fn test_class_bytes_new_with_single_range() {
    let ranges = vec![ClassBytesRange { start: 2, end: 5 }];
    let class_bytes = ClassBytes::new(ranges);
    assert_eq!(class_bytes.ranges().len(), 1);
    assert_eq!(class_bytes.ranges()[0], ClassBytesRange { start: 2, end: 5 });
}

#[test]
fn test_class_bytes_new_with_empty_ranges() {
    let ranges: Vec<ClassBytesRange> = vec![];
    let class_bytes = ClassBytes::new(ranges);
    assert_eq!(class_bytes.ranges().len(), 0);
}

