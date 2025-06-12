// Answer 0

#[test]
fn test_difference_basic() {
    let mut class_a = ClassUnicode::new(vec![ClassUnicodeRange { start: 'a', end: 'z' }]);
    let class_b = ClassUnicode::new(vec![ClassUnicodeRange { start: 'd', end: 'f' }]);
    class_a.difference(&class_b);
}

#[test]
fn test_difference_empty_case() {
    let mut class_a = ClassUnicode::new(vec![ClassUnicodeRange { start: 'a', end: 'z' }]);
    let class_b = ClassUnicode::empty();
    class_a.difference(&class_b);
}

#[test]
fn test_difference_full_range() {
    let mut class_a = ClassUnicode::new(vec![ClassUnicodeRange { start: 'a', end: 'z' }]);
    let class_b = ClassUnicode::new(vec![ClassUnicodeRange { start: 'a', end: 'z' }]);
    class_a.difference(&class_b);
}

#[test]
fn test_difference_multiple_ranges() {
    let mut class_a = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'a', end: 'z' },
        ClassUnicodeRange { start: '0', end: '9' },
    ]);
    let class_b = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'd', end: 'f' },
        ClassUnicodeRange { start: '5', end: '7' },
    ]);
    class_a.difference(&class_b);
}

#[test]
fn test_difference_with_non_overlapping_ranges() {
    let mut class_a = ClassUnicode::new(vec![ClassUnicodeRange { start: 'a', end: 'c' }]);
    let class_b = ClassUnicode::new(vec![ClassUnicodeRange { start: 'd', end: 'f' }]);
    class_a.difference(&class_b);
}

#[test]
fn test_difference_large_set() {
    let mut class_a = ClassUnicode::new((0..1000).map(|i| ClassUnicodeRange { start: char::from_u32(65 + i).unwrap(), end: char::from_u32(65 + i).unwrap() }));
    let class_b = ClassUnicode::new(vec![ClassUnicodeRange { start: 'A', end: 'Z' }]);
    class_a.difference(&class_b);
}

