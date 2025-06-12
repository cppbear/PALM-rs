// Answer 0

#[test]
fn test_difference_non_empty() {
    let mut class_bytes_a = ClassBytes::new(vec![
        ClassBytesRange { start: 0x00, end: 0x05 },
        ClassBytesRange { start: 0x10, end: 0x15 },
    ]);
    let class_bytes_b = ClassBytes::new(vec![
        ClassBytesRange { start: 0x03, end: 0x12 },
    ]);
    
    class_bytes_a.difference(&class_bytes_b);
    
    let expected_ranges = vec![
        ClassBytesRange { start: 0x00, end: 0x02 },
        ClassBytesRange { start: 0x13, end: 0x15 },
    ];
    assert_eq!(class_bytes_a.ranges(), &expected_ranges);
}

#[test]
fn test_difference_empty_other() {
    let mut class_bytes_a = ClassBytes::new(vec![
        ClassBytesRange { start: 0x00, end: 0x05 },
        ClassBytesRange { start: 0x10, end: 0x15 },
    ]);
    let class_bytes_b = ClassBytes::empty();
    
    class_bytes_a.difference(&class_bytes_b);
    
    let expected_ranges = vec![
        ClassBytesRange { start: 0x00, end: 0x05 },
        ClassBytesRange { start: 0x10, end: 0x15 },
    ];
    assert_eq!(class_bytes_a.ranges(), &expected_ranges);
}

#[test]
fn test_difference_self() {
    let mut class_bytes_a = ClassBytes::new(vec![
        ClassBytesRange { start: 0x00, end: 0x05 },
    ]);
    let class_bytes_b = ClassBytes::new(vec![
        ClassBytesRange { start: 0x00, end: 0x05 },
    ]);
    
    class_bytes_a.difference(&class_bytes_b);
    
    let expected_ranges: Vec<ClassBytesRange> = vec![];
    assert_eq!(class_bytes_a.ranges(), &expected_ranges);
}

#[test]
fn test_difference_no_overlap() {
    let mut class_bytes_a = ClassBytes::new(vec![
        ClassBytesRange { start: 0x00, end: 0x01 },
    ]);
    let class_bytes_b = ClassBytes::new(vec![
        ClassBytesRange { start: 0x02, end: 0x03 },
    ]);
    
    class_bytes_a.difference(&class_bytes_b);
    
    let expected_ranges = vec![
        ClassBytesRange { start: 0x00, end: 0x01 },
    ];
    assert_eq!(class_bytes_a.ranges(), &expected_ranges);
}

#[test]
#[should_panic]
fn test_difference_panic_empty_self() {
    let mut class_bytes_a = ClassBytes::empty();
    let class_bytes_b = ClassBytes::new(vec![
        ClassBytesRange { start: 0x00, end: 0x01 },
    ]);
    
    class_bytes_a.difference(&class_bytes_b);
}

