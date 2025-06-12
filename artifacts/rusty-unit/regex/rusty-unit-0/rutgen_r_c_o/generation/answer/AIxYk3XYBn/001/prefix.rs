// Answer 0

#[test]
fn test_union_non_empty_ranges() {
    let mut class_a = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'a', end: 'z' },
    ]);
    let class_b = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'A', end: 'Z' },
    ]);
    class_a.union(&class_b);
}

#[test]
fn test_union_empty_class() {
    let mut class_a = ClassUnicode::new(vec![]);
    let class_b = ClassUnicode::new(vec![
        ClassUnicodeRange { start: '0', end: '9' },
    ]);
    class_a.union(&class_b);
}

#[test]
fn test_union_overlapping_ranges() {
    let mut class_a = ClassUnicode::new(vec![
        ClassUnicodeRange { start: '0', end: '9' },
        ClassUnicodeRange { start: 'a', end: 'f' },
    ]);
    let class_b = ClassUnicode::new(vec![
        ClassUnicodeRange { start: '5', end: 'z' },
    ]);
    class_a.union(&class_b);
}

#[test]
fn test_union_disjoint_ranges() {
    let mut class_a = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'A', end: 'F' },
    ]);
    let class_b = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'G', end: 'Z' },
    ]);
    class_a.union(&class_b);
}

#[test]
fn test_union_with_large_intervals() {
    let mut class_a = ClassUnicode::new(vec![
        ClassUnicodeRange { start: '\u{0000}', end: '\u{007F}' },
    ]);
    let class_b = ClassUnicode::new(vec![
        ClassUnicodeRange { start: '\u{0080}', end: '\u{00FF}' },
    ]);
    class_a.union(&class_b);
}

#[test]
#[should_panic]
fn test_union_invalid_range() {
    let mut class_a = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'z', end: 'a' },
    ]);
    let class_b = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'A', end: 'Z' },
    ]);
    class_a.union(&class_b);
}

#[test]
fn test_union_empty_with_empty() {
    let mut class_a = ClassUnicode::new(vec![]);
    let class_b = ClassUnicode::new(vec![]);
    class_a.union(&class_b);
}

#[test]
fn test_union_large_number_of_ranges() {
    let mut class_a = ClassUnicode::new((0..1000).map(|i| ClassUnicodeRange {
        start: char::from_u32(i).unwrap(),
        end: char::from_u32(i).unwrap(),
    }));
    let class_b = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'a', end: 'z' },
    ]);
    class_a.union(&class_b);
}

