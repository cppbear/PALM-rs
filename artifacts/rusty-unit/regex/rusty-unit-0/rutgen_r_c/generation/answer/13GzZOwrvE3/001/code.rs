// Answer 0

#[test]
fn test_cls_byte_count_empty() {
    let cls = ClassBytes::empty();
    assert_eq!(cls_byte_count(&cls), 0);
}

#[test]
fn test_cls_byte_count_single_range() {
    let cls = ClassBytes::new(vec![ClassBytesRange { start: 1, end: 5 }]);
    assert_eq!(cls_byte_count(&cls), 5);
}

#[test]
fn test_cls_byte_count_multiple_ranges() {
    let cls = ClassBytes::new(vec![
        ClassBytesRange { start: 1, end: 3 },
        ClassBytesRange { start: 5, end: 7 },
    ]);
    assert_eq!(cls_byte_count(&cls), 5);
}

#[test]
fn test_cls_byte_count_adjacent_ranges() {
    let cls = ClassBytes::new(vec![
        ClassBytesRange { start: 1, end: 3 },
        ClassBytesRange { start: 3, end: 5 },
    ]);
    assert_eq!(cls_byte_count(&cls), 5);
}

#[test]
fn test_cls_byte_count_overlapping_ranges() {
    let cls = ClassBytes::new(vec![
        ClassBytesRange { start: 1, end: 4 },
        ClassBytesRange { start: 3, end: 5 },
    ]);
    assert_eq!(cls_byte_count(&cls), 5);
}

#[test]
fn test_cls_byte_count_no_ranges() {
    let cls = ClassBytes::new(vec![]);
    assert_eq!(cls_byte_count(&cls), 0);
}

#[test]
fn test_cls_byte_count_large_ranges() {
    let cls = ClassBytes::new(vec![
        ClassBytesRange { start: 0, end: 1000 },
        ClassBytesRange { start: 1000, end: 2000 },
    ]);
    assert_eq!(cls_byte_count(&cls), 2001);
}

