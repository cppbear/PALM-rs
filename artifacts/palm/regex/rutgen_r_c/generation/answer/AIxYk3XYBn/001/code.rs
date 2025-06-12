// Answer 0

#[test]
fn test_union_with_empty_class() {
    let mut class_a = ClassUnicode::empty();
    let class_b = ClassUnicode::empty();
    
    class_a.union(&class_b);
    
    assert_eq!(class_a.ranges(), &[]);
}

#[test]
fn test_union_with_non_empty_class() {
    let mut class_a = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'a', end: 'z' },
    ]);
    let class_b = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'A', end: 'Z' },
    ]);

    class_a.union(&class_b);
    
    assert_eq!(class_a.ranges(), &[
        ClassUnicodeRange { start: 'a', end: 'z' },
        ClassUnicodeRange { start: 'A', end: 'Z' },
    ]);
}

#[test]
fn test_union_with_overlapping_classes() {
    let mut class_a = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'a', end: 'd' },
    ]);
    let class_b = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'c', end: 'f' },
    ]);

    class_a.union(&class_b);
    
    assert_eq!(class_a.ranges(), &[
        ClassUnicodeRange { start: 'a', end: 'f' },
    ]);
}

#[test]
fn test_union_with_adjacent_classes() {
    let mut class_a = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'a', end: 'a' },
    ]);
    let class_b = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'b', end: 'b' },
    ]);

    class_a.union(&class_b);
    
    assert_eq!(class_a.ranges(), &[
        ClassUnicodeRange { start: 'a', end: 'b' },
    ]);
}

#[test]
fn test_union_with_identical_classes() {
    let mut class_a = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'x', end: 'y' },
    ]);
    let class_b = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'x', end: 'y' },
    ]);

    class_a.union(&class_b);
    
    assert_eq!(class_a.ranges(), &[
        ClassUnicodeRange { start: 'x', end: 'y' },
    ]);
}

