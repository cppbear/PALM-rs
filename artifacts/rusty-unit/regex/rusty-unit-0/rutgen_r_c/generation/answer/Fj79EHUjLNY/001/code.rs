// Answer 0

#[test]
fn test_union_with_empty_classbytes() {
    let mut class_a = ClassBytes::empty();
    let class_b = ClassBytes::empty();
    
    class_a.union(&class_b);
    
    assert_eq!(class_a.ranges(), class_b.ranges());
}

#[test]
fn test_union_with_non_empty_classbytes() {
    let range_a = ClassBytesRange { start: 0x20, end: 0x7E };
    let range_b = ClassBytesRange { start: 0x80, end: 0xFF };
    
    let mut class_a = ClassBytes::new(vec![range_a]);
    let class_b = ClassBytes::new(vec![range_b]);
    
    class_a.union(&class_b);
    
    let combined_ranges = vec![range_a, range_b];
    assert_eq!(class_a.ranges(), combined_ranges.as_slice());
}

#[test]
fn test_union_with_same_ranges() {
    let range = ClassBytesRange { start: 0x41, end: 0x5A };
    
    let mut class_a = ClassBytes::new(vec![range]);
    let class_b = ClassBytes::new(vec![range]);
    
    class_a.union(&class_b);
    
    assert_eq!(class_a.ranges(), class_b.ranges());
}

#[test]
fn test_union_with_overlapping_ranges() {
    let range_a = ClassBytesRange { start: 0x30, end: 0x39 }; // 0-9
    let range_b = ClassBytesRange { start: 0x38, end: 0x40 }; // 8-@ overlap
    
    let mut class_a = ClassBytes::new(vec![range_a]);
    let class_b = ClassBytes::new(vec![range_b]);
    
    class_a.union(&class_b);
    
    let combined_ranges = vec![ClassBytesRange { start: 0x30, end: 0x40 }];
    assert_eq!(class_a.ranges(), combined_ranges.as_slice());
} 

#[test]
fn test_union_with_full_byte_range() {
    let range_a = ClassBytesRange { start: 0x00, end: 0xFF }; // Full range
    let mut class_a = ClassBytes::new(vec![range_a]);
    let class_b = ClassBytes::empty();

    class_a.union(&class_b);
    assert_eq!(class_a.ranges(), &[range_a]);
}

