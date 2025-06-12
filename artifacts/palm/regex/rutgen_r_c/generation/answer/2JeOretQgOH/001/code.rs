// Answer 0

#[test]
fn test_cls_char_count_empty() {
    let cls = hir::ClassUnicode::empty();
    assert_eq!(cls_char_count(&cls), 0);
}

#[test]
fn test_cls_char_count_single_range() {
    let cls = hir::ClassUnicode::new(vec![hir::ClassUnicodeRange { start: 'a', end: 'a' }]);
    assert_eq!(cls_char_count(&cls), 1);
}

#[test]
fn test_cls_char_count_multiple_ranges() {
    let cls = hir::ClassUnicode::new(vec![
        hir::ClassUnicodeRange { start: 'a', end: 'c' },
        hir::ClassUnicodeRange { start: 'e', end: 'f' }
    ]);
    assert_eq!(cls_char_count(&cls), 5); // 'a', 'b', 'c', 'e', 'f'
}

#[test]
fn test_cls_char_count_overlapping_ranges() {
    let cls = hir::ClassUnicode::new(vec![
        hir::ClassUnicodeRange { start: 'a', end: 'd' },
        hir::ClassUnicodeRange { start: 'c', end: 'f' }
    ]);
    assert_eq!(cls_char_count(&cls), 6); // 'a', 'b', 'c', 'd', 'e', 'f'
}

#[test]
fn test_cls_char_count_non_contiguous_ranges() {
    let cls = hir::ClassUnicode::new(vec![
        hir::ClassUnicodeRange { start: 'a', end: 'a' },
        hir::ClassUnicodeRange { start: 'd', end: 'f' }
    ]);
    assert_eq!(cls_char_count(&cls), 4); // 'a', 'd', 'e', 'f'
}

