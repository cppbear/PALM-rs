// Answer 0

#[test]
fn test_new_ranges_non_overlapping() {
    let ranges = vec![
        ClassBytesRange { start: 0, end: 10 },
        ClassBytesRange { start: 20, end: 30 },
        ClassBytesRange { start: 50, end: 60 },
    ];
    let class_bytes = ClassBytes::new(ranges);
}

#[test]
fn test_new_ranges_overlapping() {
    let ranges = vec![
        ClassBytesRange { start: 5, end: 15 },
        ClassBytesRange { start: 10, end: 20 },
    ];
    let class_bytes = ClassBytes::new(ranges);
}

#[test]
fn test_new_ranges_adjacent() {
    let ranges = vec![
        ClassBytesRange { start: 0, end: 5 },
        ClassBytesRange { start: 5, end: 10 },
    ];
    let class_bytes = ClassBytes::new(ranges);
}

#[test]
fn test_new_ranges_single() {
    let ranges = vec![
        ClassBytesRange { start: 100, end: 150 },
    ];
    let class_bytes = ClassBytes::new(ranges);
}

#[test]
fn test_new_ranges_empty() {
    let ranges: Vec<ClassBytesRange> = vec![];
    let class_bytes = ClassBytes::new(ranges);
}

#[test]
fn test_new_ranges_full_range() {
    let ranges = vec![
        ClassBytesRange { start: 0, end: 255 },
    ];
    let class_bytes = ClassBytes::new(ranges);
}

#[test]
fn test_new_ranges_duplicate() {
    let ranges = vec![
        ClassBytesRange { start: 10, end: 20 },
        ClassBytesRange { start: 10, end: 20 },
    ];
    let class_bytes = ClassBytes::new(ranges);
}

#[test]
fn test_new_ranges_reversed() {
    let ranges = vec![
        ClassBytesRange { start: 20, end: 10 },
    ];
    let class_bytes = ClassBytes::new(ranges);
}

