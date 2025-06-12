// Answer 0

#[test]
fn test_class_bytes_iter_empty() {
    let class_bytes = ClassBytes::empty();
    let mut iter = class_bytes.iter();
    assert_eq!(iter.0.len(), 0);
}

#[test]
fn test_class_bytes_iter_single_range() {
    let range = ClassBytesRange { start: 1, end: 2 };
    let class_bytes = ClassBytes::new(vec![range]);
    let mut iter = class_bytes.iter();
    assert_eq!(iter.0.len(), 1);
    assert_eq!(iter.0[0], range);
}

#[test]
fn test_class_bytes_iter_multiple_ranges() {
    let range1 = ClassBytesRange { start: 1, end: 3 };
    let range2 = ClassBytesRange { start: 5, end: 7 };
    let class_bytes = ClassBytes::new(vec![range1, range2]);
    let mut iter = class_bytes.iter();
    assert_eq!(iter.0.len(), 2);
    assert_eq!(iter.0[0], range1);
    assert_eq!(iter.0[1], range2);
}

#[test]
fn test_class_bytes_iter_overlapping_ranges() {
    let range1 = ClassBytesRange { start: 1, end: 5 };
    let range2 = ClassBytesRange { start: 3, end: 7 };
    let class_bytes = ClassBytes::new(vec![range1, range2]);
    let mut iter = class_bytes.iter();
    assert_eq!(iter.0.len(), 2);
}

#[test]
fn test_class_bytes_iter_nested_ranges() {
    let range1 = ClassBytesRange { start: 0, end: 5 };
    let range2 = ClassBytesRange { start: 2, end: 4 };
    let class_bytes = ClassBytes::new(vec![range1, range2]);
    let mut iter = class_bytes.iter();
    assert_eq!(iter.0.len(), 2);
    assert_eq!(iter.0[0], range1);
    assert_eq!(iter.0[1], range2);
}

