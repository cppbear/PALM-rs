// Answer 0

#[test]
fn test_difference_empty_from_empty() {
    let mut class_a = ClassUnicode::empty();
    let class_b = ClassUnicode::empty();
    class_a.difference(&class_b);
    assert_eq!(class_a, ClassUnicode::empty());
}

#[test]
fn test_difference_non_empty_from_empty() {
    let mut class_a = ClassUnicode::empty();
    let class_b = ClassUnicode::new(vec![ClassUnicodeRange { start: 'a', end: 'z' }]);
    class_a.difference(&class_b);
    assert_eq!(class_a, ClassUnicode::empty());
}

#[test]
fn test_difference_empty_from_non_empty() {
    let mut class_a = ClassUnicode::new(vec![ClassUnicodeRange { start: 'a', end: 'z' }]);
    let class_b = ClassUnicode::empty();
    class_a.difference(&class_b);
    assert_eq!(class_a, ClassUnicode::new(vec![ClassUnicodeRange { start: 'a', end: 'z' }]));
}

#[test]
fn test_difference_non_overlapping_ranges() {
    let mut class_a = ClassUnicode::new(vec![ClassUnicodeRange { start: 'a', end: 'c' }]);
    let class_b = ClassUnicode::new(vec![ClassUnicodeRange { start: 'd', end: 'f' }]);
    class_a.difference(&class_b);
    assert_eq!(class_a, ClassUnicode::new(vec![ClassUnicodeRange { start: 'a', end: 'c' }]));
}

#[test]
fn test_difference_overlapping_ranges() {
    let mut class_a = ClassUnicode::new(vec![ClassUnicodeRange { start: 'a', end: 'd' }]);
    let class_b = ClassUnicode::new(vec![ClassUnicodeRange { start: 'b', end: 'c' }]);
    class_a.difference(&class_b);
    assert_eq!(class_a, ClassUnicode::new(vec![ClassUnicodeRange { start: 'a', end: 'a' }, ClassUnicodeRange { start: 'd', end: 'd' }]));
}

#[test]
fn test_difference_entire_range() {
    let mut class_a = ClassUnicode::new(vec![ClassUnicodeRange { start: 'a', end: 'd' }]);
    let class_b = ClassUnicode::new(vec![ClassUnicodeRange { start: 'a', end: 'd' }]);
    class_a.difference(&class_b);
    assert_eq!(class_a, ClassUnicode::empty());
}

