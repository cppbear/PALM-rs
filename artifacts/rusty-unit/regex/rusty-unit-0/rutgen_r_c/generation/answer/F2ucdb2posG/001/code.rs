// Answer 0

#[test]
fn test_hir_class_empty_ranges() {
    let ranges: &[(char, char)] = &[];
    let result = hir_class(ranges);
    let expected = hir::ClassUnicode::new(vec![]);
    assert_eq!(result, expected);
}

#[test]
fn test_hir_class_single_range() {
    let ranges: &[(char, char)] = &[('a', 'z')];
    let result = hir_class(ranges);
    let expected = hir::ClassUnicode::new(vec![hir::ClassUnicodeRange::new('a', 'z')]);
    assert_eq!(result, expected);
}

#[test]
fn test_hir_class_multiple_ranges() {
    let ranges: &[(char, char)] = &[('a', 'c'), ('f', 'g'), ('m', 'p')];
    let result = hir_class(ranges);
    let expected = hir::ClassUnicode::new(vec![
        hir::ClassUnicodeRange::new('a', 'c'),
        hir::ClassUnicodeRange::new('f', 'g'),
        hir::ClassUnicodeRange::new('m', 'p')
    ]);
    assert_eq!(result, expected);
}

#[test]
fn test_hir_class_overlapping_ranges() {
    let ranges: &[(char, char)] = &[('a', 'd'), ('c', 'f')];
    let result = hir_class(ranges);
    let expected = hir::ClassUnicode::new(vec![
        hir::ClassUnicodeRange::new('a', 'd'),
        hir::ClassUnicodeRange::new('c', 'f')
    ]);
    assert_eq!(result, expected);
}

#[test]
fn test_hir_class_invalid_ranges() {
    let ranges: &[(char, char)] = &[('z', 'a')];
    // This test might panic or return an error depending on implementation details.
    let result = hir_class(ranges);
    let expected = hir::ClassUnicode::new(vec![hir::ClassUnicodeRange::new('z', 'a')]);
    // Ensure it matches a non-empty result, although logically this may not be valid.
    assert_eq!(result.set.len(), 1); 
    assert_eq!(result.set.contains_range(ClassUnicodeRange{ start: 'z', end: 'a' }), true);
}

