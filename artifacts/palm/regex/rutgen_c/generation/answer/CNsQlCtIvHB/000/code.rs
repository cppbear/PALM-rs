// Answer 0

#[test]
fn test_class_bytes_iter_empty() {
    let class_bytes = ClassBytes::empty();
    let mut iter = class_bytes.iter();
    assert!(iter.0.0.len() == 0);
}

#[test]
fn test_class_bytes_iter_single_range() {
    let range = ClassBytesRange { start: 0, end: 1 };
    let class_bytes = ClassBytes::new(vec![range]);
    let mut iter = class_bytes.iter();
    assert!(iter.0.0.len() == 1);
    assert_eq!(iter.0.0[0], range);
}

#[test]
fn test_class_bytes_iter_multiple_ranges() {
    let ranges = vec![
        ClassBytesRange { start: 1, end: 2 },
        ClassBytesRange { start: 3, end: 5 },
    ];
    let class_bytes = ClassBytes::new(ranges.clone());
    let mut iter = class_bytes.iter();
    assert!(iter.0.0.len() == 2);
    assert_eq!(iter.0.0[0], ranges[0]);
    assert_eq!(iter.0.0[1], ranges[1]);
}

