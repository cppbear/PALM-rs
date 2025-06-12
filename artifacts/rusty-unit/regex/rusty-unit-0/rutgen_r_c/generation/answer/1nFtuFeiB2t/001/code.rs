// Answer 0

#[test]
fn test_symmetric_difference_non_empty_classes() {
    let mut class1 = ClassUnicode::new(vec![ClassUnicodeRange { start: 'a', end: 'b' }, ClassUnicodeRange { start: 'c', end: 'd' }]);
    let class2 = ClassUnicode::new(vec![ClassUnicodeRange { start: 'b', end: 'c' }, ClassUnicodeRange { start: 'e', end: 'f' }]);

    class1.symmetric_difference(&class2);
    let expected_ranges = vec![ClassUnicodeRange { start: 'a', end: 'b' }, ClassUnicodeRange { start: 'c', end: 'd' }, ClassUnicodeRange { start: 'e', end: 'f' }];
    
    assert_eq!(class1.ranges(), &expected_ranges);
}

#[test]
fn test_symmetric_difference_empty_first_class() {
    let mut class1 = ClassUnicode::empty();
    let class2 = ClassUnicode::new(vec![ClassUnicodeRange { start: 'a', end: 'b' }]);

    class1.symmetric_difference(&class2);
    let expected_ranges = vec![ClassUnicodeRange { start: 'a', end: 'b' }];
    
    assert_eq!(class1.ranges(), &expected_ranges);
}

#[test]
fn test_symmetric_difference_empty_second_class() {
    let mut class1 = ClassUnicode::new(vec![ClassUnicodeRange { start: 'a', end: 'b' }]);
    let class2 = ClassUnicode::empty();

    class1.symmetric_difference(&class2);
    let expected_ranges = vec![ClassUnicodeRange { start: 'a', end: 'b' }];
    
    assert_eq!(class1.ranges(), &expected_ranges);
}

#[test]
fn test_symmetric_difference_identical_classes() {
    let mut class1 = ClassUnicode::new(vec![ClassUnicodeRange { start: 'a', end: 'b' }]);
    let class2 = ClassUnicode::new(vec![ClassUnicodeRange { start: 'a', end: 'b' }]);

    class1.symmetric_difference(&class2);
    let expected_ranges: Vec<ClassUnicodeRange> = vec![];
    
    assert_eq!(class1.ranges(), &expected_ranges);
}

#[test]
fn test_symmetric_difference_with_full_overlap() {
    let mut class1 = ClassUnicode::new(vec![ClassUnicodeRange { start: 'a', end: 'z' }]);
    let class2 = ClassUnicode::new(vec![ClassUnicodeRange { start: 'd', end: 'f' }, ClassUnicodeRange { start: 'g', end: 'j' }]);

    class1.symmetric_difference(&class2);
    let expected_ranges = vec![ClassUnicodeRange { start: 'a', end: 'c' }, ClassUnicodeRange { start: 'h', end: 'z' }];
    
    assert_eq!(class1.ranges(), &expected_ranges);
}

