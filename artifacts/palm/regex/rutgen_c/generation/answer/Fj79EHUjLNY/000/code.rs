// Answer 0

#[test]
fn test_class_bytes_union() {
    // Initialize two ClassBytes instances with different ranges
    let range1 = ClassBytesRange { start: 1, end: 5 };
    let range2 = ClassBytesRange { start: 6, end: 10 };
    
    let class_bytes1 = ClassBytes::new(vec![range1]);
    let class_bytes2 = ClassBytes::new(vec![range2]);
    
    let mut class_bytes_to_union = class_bytes1.clone();

    // Perform the union operation
    class_bytes_to_union.union(&class_bytes2);

    // Check the combined result should include all ranges
    let expected_ranges = vec![range1, range2];
    assert_eq!(class_bytes_to_union.ranges(), &expected_ranges);
}

#[test]
fn test_class_bytes_union_with_empty() {
    let range = ClassBytesRange { start: 1, end: 3 };
    
    let class_bytes1 = ClassBytes::new(vec![range]);
    let class_bytes2 = ClassBytes::empty(); // Empty ClassBytes

    let mut class_bytes_to_union = class_bytes1.clone();

    // Perform the union operation with an empty ClassBytes
    class_bytes_to_union.union(&class_bytes2);

    // The result should be the same as class_bytes1
    assert_eq!(class_bytes_to_union.ranges(), class_bytes1.ranges());
}

