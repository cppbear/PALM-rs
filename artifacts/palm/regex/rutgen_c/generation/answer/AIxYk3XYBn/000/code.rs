// Answer 0

#[test]
fn test_union_with_non_empty() {
    let mut class_a = ClassUnicode::new(vec![ClassUnicodeRange { start: 'a', end: 'b' }]);
    let class_b = ClassUnicode::new(vec![ClassUnicodeRange { start: 'c', end: 'd' }]);
    
    class_a.union(&class_b);
    
    let expected_ranges = vec![
        ClassUnicodeRange { start: 'a', end: 'b' },
        ClassUnicodeRange { start: 'c', end: 'd' },
    ];
    
    assert_eq!(class_a.ranges(), &expected_ranges);
}

#[test]
fn test_union_with_empty() {
    let mut class_a = ClassUnicode::new(vec![ClassUnicodeRange { start: 'a', end: 'b' }]);
    let class_b = ClassUnicode::empty();
    
    class_a.union(&class_b);
    
    let expected_ranges = vec![
        ClassUnicodeRange { start: 'a', end: 'b' },
    ];
    
    assert_eq!(class_a.ranges(), &expected_ranges);
}

#[test]
fn test_union_with_identical() {
    let mut class_a = ClassUnicode::new(vec![ClassUnicodeRange { start: 'a', end: 'b' }]);
    let class_b = ClassUnicode::new(vec![ClassUnicodeRange { start: 'a', end: 'b' }]);
    
    class_a.union(&class_b);
    
    let expected_ranges = vec![
        ClassUnicodeRange { start: 'a', end: 'b' },
    ];
    
    assert_eq!(class_a.ranges(), &expected_ranges);
}

