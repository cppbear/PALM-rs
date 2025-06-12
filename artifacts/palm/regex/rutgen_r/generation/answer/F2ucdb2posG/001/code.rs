// Answer 0

#[test]
fn test_hir_class_empty_ranges() {
    let ranges: &[(char, char)] = &[];
    let result = hir_class(ranges);
    assert!(result.ranges.is_empty());
}

#[test]
fn test_hir_class_single_range() {
    let ranges: &[(char, char)] = &[('a', 'z')];
    let result = hir_class(ranges);
    assert_eq!(result.ranges.len(), 1);
    assert_eq!(result.ranges[0].start, 'a');
    assert_eq!(result.ranges[0].end, 'z');
}

#[test]
fn test_hir_class_multiple_ranges() {
    let ranges: &[(char, char)] = &[('a', 'c'), ('f', 'h')];
    let result = hir_class(ranges);
    assert_eq!(result.ranges.len(), 2);
    assert_eq!(result.ranges[0].start, 'a');
    assert_eq!(result.ranges[0].end, 'c');
    assert_eq!(result.ranges[1].start, 'f');
    assert_eq!(result.ranges[1].end, 'h');
}

#[test]
fn test_hir_class_overlapping_ranges() {
    let ranges: &[(char, char)] = &[('a', 'd'), ('c', 'f')];
    let result = hir_class(ranges);
    assert_eq!(result.ranges.len(), 2); 
    assert_eq!(result.ranges[0].start, 'a');
    assert_eq!(result.ranges[0].end, 'd');
    assert_eq!(result.ranges[1].start, 'c');
    assert_eq!(result.ranges[1].end, 'f');
}

#[test]
fn test_hir_class_single_char_range() {
    let ranges: &[(char, char)] = &[('x', 'x')];
    let result = hir_class(ranges);
    assert_eq!(result.ranges.len(), 1);
    assert_eq!(result.ranges[0].start, 'x');
    assert_eq!(result.ranges[0].end, 'x');
}

