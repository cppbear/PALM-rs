// Answer 0

#[test]
fn test_push_valid_range() {
    let mut class_bytes = ClassBytes::empty();
    let valid_range = ClassBytesRange { start: 10, end: 20 };
    class_bytes.push(valid_range);
    
    assert_eq!(class_bytes.ranges().len(), 1);
    assert_eq!(class_bytes.ranges()[0], valid_range);
}

#[test]
#[should_panic] // testing panic condition due to invalid range (end < start)
fn test_push_invalid_range() {
    let mut class_bytes = ClassBytes::empty();
    let invalid_range = ClassBytesRange { start: 30, end: 20 }; // Invalid range
    class_bytes.push(invalid_range);
}

#[test]
fn test_push_multiple_ranges() {
    let mut class_bytes = ClassBytes::empty();
    let range1 = ClassBytesRange { start: 5, end: 10 };
    let range2 = ClassBytesRange { start: 15, end: 25 };
    
    class_bytes.push(range1);
    class_bytes.push(range2);
    
    assert_eq!(class_bytes.ranges().len(), 2);
    assert_eq!(class_bytes.ranges()[0], range1);
    assert_eq!(class_bytes.ranges()[1], range2);
}

#[test]
fn test_push_overlapping_ranges() {
    let mut class_bytes = ClassBytes::empty();
    let range1 = ClassBytesRange { start: 5, end: 15 };
    let overlapping_range = ClassBytesRange { start: 10, end: 20 };
    
    class_bytes.push(range1);
    class_bytes.push(overlapping_range);
    
    assert_eq!(class_bytes.ranges().len(), 2);
    assert_eq!(class_bytes.ranges()[1], overlapping_range);
}

