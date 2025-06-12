// Answer 0

#[test]
fn test_hir_class_empty() {
    let ranges: &[(char, char)] = &[];
    let result = hir_class(ranges);
    assert_eq!(result, hir::ClassUnicode::new(Vec::new()));
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
    let ranges: &[(char, char)] = &[('a', 'z'), ('A', 'Z')];
    let result = hir_class(ranges);
    let expected = hir::ClassUnicode::new(vec![
        hir::ClassUnicodeRange::new('a', 'z'),
        hir::ClassUnicodeRange::new('A', 'Z'),
    ]);
    assert_eq!(result, expected);
}

#[test]
fn test_hir_class_overlapping_ranges() {
    let ranges: &[(char, char)] = &[('a', 'c'), ('b', 'd')];
    let result = hir_class(ranges);
    let expected = hir::ClassUnicode::new(vec![
        hir::ClassUnicodeRange::new('a', 'c'),
        hir::ClassUnicodeRange::new('b', 'd'),
    ]);
    assert_eq!(result, expected);
}

#[test]
fn test_hir_class_same_start_end() {
    let ranges: &[(char, char)] = &[('x', 'x')];
    let result = hir_class(ranges);
    let expected = hir::ClassUnicode::new(vec![hir::ClassUnicodeRange::new('x', 'x')]);
    assert_eq!(result, expected);
}

