// Answer 0

#[test]
fn test_symmetric_difference_with_no_overlap() {
    let mut class_a = ClassUnicode::new(vec![ClassUnicodeRange { start: 'a', end: 'b' }]);
    let class_b = ClassUnicode::new(vec![ClassUnicodeRange { start: 'c', end: 'd' }]);
    
    class_a.symmetric_difference(&class_b);

    assert_eq!(class_a.ranges(), &[ClassUnicodeRange { start: 'a', end: 'b' }, ClassUnicodeRange { start: 'c', end: 'd' }]);
}

#[test]
fn test_symmetric_difference_with_full_overlap() {
    let mut class_a = ClassUnicode::new(vec![ClassUnicodeRange { start: 'a', end: 'b' }]);
    let class_b = ClassUnicode::new(vec![ClassUnicodeRange { start: 'a', end: 'b' }]);
    
    class_a.symmetric_difference(&class_b);
    
    assert!(class_a.ranges().is_empty());
}

#[test]
fn test_symmetric_difference_with_partial_overlap() {
    let mut class_a = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'a', end: 'c' },
    ]);
    let class_b = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'b', end: 'd' },
    ]);

    class_a.symmetric_difference(&class_b);
    
    assert_eq!(class_a.ranges(), &[ClassUnicodeRange { start: 'a', end: 'b' }, ClassUnicodeRange { start: 'c', end: 'd' }]);
}

#[test]
fn test_symmetric_difference_empty_class() {
    let mut class_a = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'a', end: 'b' },
    ]);
    let class_b = ClassUnicode::empty();

    class_a.symmetric_difference(&class_b);

    assert_eq!(class_a.ranges(), &[ClassUnicodeRange { start: 'a', end: 'b' }]);
}

#[test]
fn test_symmetric_difference_multiple_ranges() {
    let mut class_a = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'a', end: 'c' },
        ClassUnicodeRange { start: 'e', end: 'g' },
    ]);
    let class_b = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'b', end: 'd' },
    ]);

    class_a.symmetric_difference(&class_b);
    
    assert_eq!(class_a.ranges(), &[
        ClassUnicodeRange { start: 'a', end: 'b' },
        ClassUnicodeRange { start: 'c', end: 'c' },
        ClassUnicodeRange { start: 'e', end: 'g' }
    ]);
}

