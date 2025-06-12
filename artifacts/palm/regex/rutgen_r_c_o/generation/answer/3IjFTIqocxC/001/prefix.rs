// Answer 0

#[test]
fn test_difference_non_empty_overlapping_ranges() {
    let mut class_bytes_a = ClassBytes::new(vec![
        ClassBytesRange { start: 0, end: 50 },
        ClassBytesRange { start: 100, end: 150 },
    ]);
    let class_bytes_b = ClassBytes::new(vec![
        ClassBytesRange { start: 40, end: 110 },
    ]);
    class_bytes_a.difference(&class_bytes_b);
}

#[test]
fn test_difference_empty_other_set() {
    let mut class_bytes_a = ClassBytes::new(vec![
        ClassBytesRange { start: 0, end: 100 },
    ]);
    let class_bytes_b = ClassBytes::empty();
    class_bytes_a.difference(&class_bytes_b);
}

#[test]
fn test_difference_no_overlap() {
    let mut class_bytes_a = ClassBytes::new(vec![
        ClassBytesRange { start: 0, end: 10 },
    ]);
    let class_bytes_b = ClassBytes::new(vec![
        ClassBytesRange { start: 11, end: 20 },
    ]);
    class_bytes_a.difference(&class_bytes_b);
}

#[test]
fn test_difference_complete_overlap() {
    let mut class_bytes_a = ClassBytes::new(vec![
        ClassBytesRange { start: 0, end: 255 },
    ]);
    let class_bytes_b = ClassBytes::new(vec![
        ClassBytesRange { start: 0, end: 255 },
    ]);
    class_bytes_a.difference(&class_bytes_b);
}

#[test]
fn test_difference_partial_overlap() {
    let mut class_bytes_a = ClassBytes::new(vec![
        ClassBytesRange { start: 50, end: 200 },
    ]);
    let class_bytes_b = ClassBytes::new(vec![
        ClassBytesRange { start: 100, end: 150 },
    ]);
    class_bytes_a.difference(&class_bytes_b);
}

#[test]
fn test_difference_multiple_ranged_non_full_overlap() {
    let mut class_bytes_a = ClassBytes::new(vec![
        ClassBytesRange { start: 0, end: 50 },
        ClassBytesRange { start: 51, end: 100 },
    ]);
    let class_bytes_b = ClassBytes::new(vec![
        ClassBytesRange { start: 30, end: 70 },
    ]);
    class_bytes_a.difference(&class_bytes_b);
}

