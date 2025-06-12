// Answer 0

#[test]
fn test_negate_empty_class_bytes() {
    let mut class_bytes = ClassBytes::empty();
    class_bytes.negate();
}

#[test]
fn test_negate_single_range() {
    let range = ClassBytesRange { start: 10, end: 10 };
    let mut class_bytes = ClassBytes::new(vec![range]);
    class_bytes.negate();
}

#[test]
fn test_negate_multiple_ranges() {
    let ranges = vec![
        ClassBytesRange { start: 0, end: 5 },
        ClassBytesRange { start: 20, end: 30 },
    ];
    let mut class_bytes = ClassBytes::new(ranges);
    class_bytes.negate();
}

#[test]
fn test_negate_overlapping_ranges() {
    let ranges = vec![
        ClassBytesRange { start: 5, end: 15 },
        ClassBytesRange { start: 10, end: 20 },
    ];
    let mut class_bytes = ClassBytes::new(ranges);
    class_bytes.negate();
}

#[test]
fn test_negate_full_byte_range() {
    let range = ClassBytesRange { start: 0, end: 255 };
    let mut class_bytes = ClassBytes::new(vec![range]);
    class_bytes.negate();
}

#[test]
fn test_negate_range_with_empty_space() {
    let ranges = vec![
        ClassBytesRange { start: 0, end: 1 },
        ClassBytesRange { start: 3, end: 4 },
    ];
    let mut class_bytes = ClassBytes::new(ranges);
    class_bytes.negate();
}

#[test]
#[should_panic]
fn test_negate_invalid_range() {
    let range = ClassBytesRange { start: 5, end: 3 };
    let mut class_bytes = ClassBytes::new(vec![range]);
    class_bytes.negate();
}

