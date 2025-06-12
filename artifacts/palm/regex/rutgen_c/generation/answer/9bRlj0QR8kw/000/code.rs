// Answer 0

#[test]
fn test_class_bytes_negate_empty() {
    let mut class_bytes = ClassBytes::empty();
    class_bytes.negate();
    assert_eq!(class_bytes.ranges().len(), 1);
    assert_eq!(class_bytes.ranges()[0].start, 0);
    assert_eq!(class_bytes.ranges()[0].end, 255);
}

#[test]
fn test_class_bytes_negate_non_empty() {
    let range1 = ClassBytesRange { start: 10, end: 20 };
    let range2 = ClassBytesRange { start: 30, end: 40 };
    let mut class_bytes = ClassBytes::new(vec![range1, range2]);
    
    class_bytes.negate();
    
    assert_eq!(class_bytes.ranges().len(), 3);
    assert_eq!(class_bytes.ranges()[0].start, 0);
    assert_eq!(class_bytes.ranges()[0].end, 9);
    assert_eq!(class_bytes.ranges()[1].start, 21);
    assert_eq!(class_bytes.ranges()[1].end, 29);
    assert_eq!(class_bytes.ranges()[2].start, 41);
    assert_eq!(class_bytes.ranges()[2].end, 255);
}

#[test]
fn test_class_bytes_negate_full_range() {
    let mut class_bytes = ClassBytes::new(vec![ClassBytesRange { start: 0, end: 255 }]);
    
    class_bytes.negate();
    
    assert_eq!(class_bytes.ranges().len(), 1);
    assert_eq!(class_bytes.ranges()[0].start, 0);
    assert_eq!(class_bytes.ranges()[0].end, 255);
}

#[test]
fn test_class_bytes_negate_single_range_gaps() {
    let range1 = ClassBytesRange { start: 1, end: 2 };
    let mut class_bytes = ClassBytes::new(vec![range1]);
    
    class_bytes.negate();
    
    assert_eq!(class_bytes.ranges().len(), 2);
    assert_eq!(class_bytes.ranges()[0].start, 0);
    assert_eq!(class_bytes.ranges()[0].end, 0);
    assert_eq!(class_bytes.ranges()[1].start, 3);
    assert_eq!(class_bytes.ranges()[1].end, 255);
}

