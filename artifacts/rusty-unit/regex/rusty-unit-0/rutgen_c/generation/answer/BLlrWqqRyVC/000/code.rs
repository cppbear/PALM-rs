// Answer 0

#[test]
fn test_is_all_ascii_empty() {
    struct DummyInterval;

    impl Interval for DummyInterval {}

    let empty_class_bytes = ClassBytes::empty();
    assert!(empty_class_bytes.is_all_ascii());
}

#[test]
fn test_is_all_ascii_with_ascii() {
    struct DummyInterval;

    impl Interval for DummyInterval {}

    let range = ClassBytesRange { start: 0u8, end: 127u8 };
    let intervals = IntervalSet::new(vec![range]);
    let class_bytes = ClassBytes { set: intervals };
    assert!(class_bytes.is_all_ascii());
}

#[test]
fn test_is_all_ascii_with_non_ascii() {
    struct DummyInterval;

    impl Interval for DummyInterval {}

    let range = ClassBytesRange { start: 128u8, end: 255u8 };
    let intervals = IntervalSet::new(vec![range]);
    let class_bytes = ClassBytes { set: intervals };
    assert!(!class_bytes.is_all_ascii());
}

#[test]
fn test_is_all_ascii_with_multiple_ranges() {
    struct DummyInterval;

    impl Interval for DummyInterval {}

    let range1 = ClassBytesRange { start: 0u8, end: 100u8 };
    let range2 = ClassBytesRange { start: 101u8, end: 127u8 };
    let intervals = IntervalSet::new(vec![range1, range2]);
    let class_bytes = ClassBytes { set: intervals };
    assert!(class_bytes.is_all_ascii());

    let range3 = ClassBytesRange { start: 128u8, end: 255u8 };
    intervals.push(range3);
    assert!(!class_bytes.is_all_ascii());
}

