// Answer 0

#[test]
fn test_new_with_single_range() {
    let ranges = vec![ClassBytesRange { start: 0, end: 10 }];
    let class_bytes = ClassBytes::new(ranges);
    assert_eq!(class_bytes.ranges().len(), 1);
    assert_eq!(class_bytes.ranges()[0], ClassBytesRange { start: 0, end: 10 });
}

#[test]
fn test_new_with_multiple_non_overlapping_ranges() {
    let ranges = vec![
        ClassBytesRange { start: 0, end: 10 },
        ClassBytesRange { start: 20, end: 30 },
    ];
    let class_bytes = ClassBytes::new(ranges);
    assert_eq!(class_bytes.ranges().len(), 2);
    assert_eq!(class_bytes.ranges()[0], ClassBytesRange { start: 0, end: 10 });
    assert_eq!(class_bytes.ranges()[1], ClassBytesRange { start: 20, end: 30 });
}

#[test]
fn test_new_with_overlapping_ranges() {
    let ranges = vec![
        ClassBytesRange { start: 0, end: 10 },
        ClassBytesRange { start: 5, end: 15 },
    ];
    let class_bytes = ClassBytes::new(ranges);
    assert_eq!(class_bytes.ranges().len(), 2);
    assert_eq!(class_bytes.ranges()[0], ClassBytesRange { start: 0, end: 10 });
    assert_eq!(class_bytes.ranges()[1], ClassBytesRange { start: 5, end: 15 });
}

#[test]
fn test_new_with_empty_ranges() {
    let ranges: Vec<ClassBytesRange> = vec![];
    let class_bytes = ClassBytes::new(ranges);
    assert_eq!(class_bytes.ranges().len(), 0);
}

#[test]
fn test_new_with_a_single_full_range() {
    let ranges = vec![ClassBytesRange { start: u8::MIN, end: u8::MAX }];
    let class_bytes = ClassBytes::new(ranges);
    assert_eq!(class_bytes.ranges().len(), 1);
    assert_eq!(class_bytes.ranges()[0], ClassBytesRange { start: u8::MIN, end: u8::MAX });
}

