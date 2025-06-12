// Answer 0

#[test]
fn test_new_unicode_range_with_valid_characters() {
    let range = regex_syntax::hir::ClassUnicodeRange::new('a', 'z');
    assert_eq!(range.start, 'a');
    assert_eq!(range.end, 'z');
}

#[test]
fn test_new_unicode_range_with_equal_characters() {
    let range = regex_syntax::hir::ClassUnicodeRange::new('b', 'b');
    assert_eq!(range.start, 'b');
    assert_eq!(range.end, 'b');
}

#[test]
fn test_new_unicode_range_with_reversed_characters() {
    let range = regex_syntax::hir::ClassUnicodeRange::new('z', 'a');
    assert_eq!(range.start, 'a');
    assert_eq!(range.end, 'z');
}

