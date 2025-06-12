// Answer 0

#[test]
fn test_is_all_ascii_with_empty_class() {
    let class_bytes = ClassBytes::empty();
    assert!(class_bytes.is_all_ascii());
}

#[test]
fn test_is_all_ascii_with_only_ascii_ranges() {
    let ranges = vec![
        ClassBytesRange { start: 0, end: 127 },
    ];
    let class_bytes = ClassBytes::new(ranges);
    assert!(class_bytes.is_all_ascii());
}

#[test]
fn test_is_all_ascii_with_non_ascii_range() {
    let ranges = vec![
        ClassBytesRange { start: 128, end: 255 },
    ];
    let class_bytes = ClassBytes::new(ranges);
    assert!(!class_bytes.is_all_ascii());
}

#[test]
fn test_is_all_ascii_with_mixed_ranges() {
    let ranges = vec![
        ClassBytesRange { start: 0, end: 127 },
        ClassBytesRange { start: 128, end: 255 },
    ];
    let class_bytes = ClassBytes::new(ranges);
    assert!(!class_bytes.is_all_ascii());
}

#[test]
fn test_is_all_ascii_with_overlapping_ranges() {
    let ranges = vec![
        ClassBytesRange { start: 0, end: 150 },
    ];
    let class_bytes = ClassBytes::new(ranges);
    assert!(!class_bytes.is_all_ascii());
}

#[test]
fn test_is_all_ascii_with_ranges_containing_zero() {
    let ranges = vec![
        ClassBytesRange { start: 0, end: 100 },
        ClassBytesRange { start: 200, end: 255 },
    ];
    let class_bytes = ClassBytes::new(ranges);
    assert!(!class_bytes.is_all_ascii());
}

#[test]
fn test_is_all_ascii_with_single_non_ascii_range() {
    let ranges = vec![
        ClassBytesRange { start: 255, end: 255 },
    ];
    let class_bytes = ClassBytes::new(ranges);
    assert!(!class_bytes.is_all_ascii());
}

