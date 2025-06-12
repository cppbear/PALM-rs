// Answer 0

#[test]
fn test_is_all_ascii_with_empty_class_bytes() {
    let class_bytes = ClassBytes::empty();
    class_bytes.is_all_ascii();
}

#[test]
fn test_is_all_ascii_with_single_ascii_range() {
    let range = ClassBytesRange { start: 0, end: 127 };
    let class_bytes = ClassBytes::new(vec![range]);
    class_bytes.is_all_ascii();
}

#[test]
fn test_is_all_ascii_with_single_non_ascii_range() {
    let range = ClassBytesRange { start: 128, end: 255 };
    let class_bytes = ClassBytes::new(vec![range]);
    class_bytes.is_all_ascii();
}

#[test]
fn test_is_all_ascii_with_multiple_ranges_all_ascii() {
    let ranges = vec![
        ClassBytesRange { start: 0, end: 63 },
        ClassBytesRange { start: 64, end: 127 },
    ];
    let class_bytes = ClassBytes::new(ranges);
    class_bytes.is_all_ascii();
}

#[test]
fn test_is_all_ascii_with_multiple_ranges_with_non_ascii() {
    let ranges = vec![
        ClassBytesRange { start: 0, end: 127 },
        ClassBytesRange { start: 128, end: 255 },
    ];
    let class_bytes = ClassBytes::new(ranges);
    class_bytes.is_all_ascii();
}

#[test]
fn test_is_all_ascii_with_non_contiguous_ascii_ranges() {
    let ranges = vec![
        ClassBytesRange { start: 0, end: 31 },
        ClassBytesRange { start: 32, end: 127 },
    ];
    let class_bytes = ClassBytes::new(ranges);
    class_bytes.is_all_ascii();
}

