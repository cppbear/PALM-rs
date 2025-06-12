// Answer 0

#[test]
fn test_symmetric_difference_non_empty_sets() {
    let mut class_a = ClassBytes::new(vec![
        ClassBytesRange { start: 0x41, end: 0x41 }, // 'A'
        ClassBytesRange { start: 0x42, end: 0x42 }, // 'B'
    ]);
    
    let class_b = ClassBytes::new(vec![
        ClassBytesRange { start: 0x42, end: 0x42 }, // 'B'
        ClassBytesRange { start: 0x43, end: 0x43 }, // 'C'
    ]);
    
    class_a.symmetric_difference(&class_b);
    let expected_ranges: Vec<ClassBytesRange> = vec![
        ClassBytesRange { start: 0x41, end: 0x41 }, // 'A'
        ClassBytesRange { start: 0x43, end: 0x43 }, // 'C'
    ];
    
    assert_eq!(class_a.ranges(), expected_ranges.as_slice());
}

#[test]
fn test_symmetric_difference_empty_self() {
    let mut class_a = ClassBytes::empty();
    
    let class_b = ClassBytes::new(vec![
        ClassBytesRange { start: 0x41, end: 0x41 }, // 'A'
        ClassBytesRange { start: 0x42, end: 0x42 }, // 'B'
    ]);
    
    class_a.symmetric_difference(&class_b);
    
    let expected_ranges: Vec<ClassBytesRange> = vec![
        ClassBytesRange { start: 0x41, end: 0x41 }, // 'A'
        ClassBytesRange { start: 0x42, end: 0x42 }, // 'B'
    ];
    
    assert_eq!(class_a.ranges(), expected_ranges.as_slice());
}

#[test]
fn test_symmetric_difference_empty_other() {
    let mut class_a = ClassBytes::new(vec![
        ClassBytesRange { start: 0x41, end: 0x41 }, // 'A'
        ClassBytesRange { start: 0x42, end: 0x42 }, // 'B'
    ]);
    
    let class_b = ClassBytes::empty();
    
    class_a.symmetric_difference(&class_b);
    
    let expected_ranges: Vec<ClassBytesRange> = vec![
        ClassBytesRange { start: 0x41, end: 0x41 }, // 'A'
        ClassBytesRange { start: 0x42, end: 0x42 }, // 'B'
    ];
    
    assert_eq!(class_a.ranges(), expected_ranges.as_slice());
}

#[test]
fn test_symmetric_difference_identical_sets() {
    let mut class_a = ClassBytes::new(vec![
        ClassBytesRange { start: 0x41, end: 0x41 }, // 'A'
        ClassBytesRange { start: 0x42, end: 0x42 }, // 'B'
    ]);
    
    let class_b = ClassBytes::new(vec![
        ClassBytesRange { start: 0x41, end: 0x41 }, // 'A'
        ClassBytesRange { start: 0x42, end: 0x42 }, // 'B'
    ]);
    
    class_a.symmetric_difference(&class_b);
    
    let expected_ranges: Vec<ClassBytesRange> = vec![];
    
    assert_eq!(class_a.ranges(), expected_ranges.as_slice());
}

