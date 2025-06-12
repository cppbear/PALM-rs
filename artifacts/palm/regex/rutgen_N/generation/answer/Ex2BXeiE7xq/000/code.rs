// Answer 0

#[derive(Debug, PartialEq)]
struct ClassBytesRange {
    start: u8,
    end: u8,
}

impl ClassBytesRange {
    fn create(start: u8, end: u8) -> ClassBytesRange {
        assert!(start <= end, "start must be less than or equal to end");
        ClassBytesRange { start, end }
    }
}

#[test]
fn test_class_bytes_range_creation_valid() {
    let range = ClassBytesRange::create(0, 255);
    assert_eq!(range.start, 0);
    assert_eq!(range.end, 255);
}

#[test]
#[should_panic(expected = "start must be less than or equal to end")]
fn test_class_bytes_range_creation_invalid_start_greater_than_end() {
    ClassBytesRange::create(100, 50);
}

#[test]
#[should_panic(expected = "start must be less than or equal to end")]
fn test_class_bytes_range_creation_invalid_start_equal_to_end() {
    let range = ClassBytesRange::create(100, 100);
    assert_eq!(range.start, range.end);
}

