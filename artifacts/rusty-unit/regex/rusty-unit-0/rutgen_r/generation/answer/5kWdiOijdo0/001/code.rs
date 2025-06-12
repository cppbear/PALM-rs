// Answer 0

#[test]
fn test_new_unicode_scalar_value_range_valid_range() {
    let range = regex_syntax::hir::ClassUnicodeRange::new('a', 'z');
    assert_eq!(range.start(), 'a');
    assert_eq!(range.end(), 'z');
}

#[test]
fn test_new_unicode_scalar_value_range_equal_start_end() {
    let range = regex_syntax::hir::ClassUnicodeRange::new('a', 'a');
    assert_eq!(range.start(), 'a');
    assert_eq!(range.end(), 'a');
}

#[test]
#[should_panic]
fn test_new_unicode_scalar_value_range_invalid_range() {
    // This should panic because start is greater than end
    let _range = regex_syntax::hir::ClassUnicodeRange::new('z', 'a');
}

#[test]
fn test_new_unicode_scalar_value_range_max_boundary() {
    let range = regex_syntax::hir::ClassUnicodeRange::new('\u{10FFFD}', '\u{10FFFF}');
    assert_eq!(range.start(), '\u{10FFFD}');
    assert_eq!(range.end(), '\u{10FFFF}');
}

#[test]
fn test_new_unicode_scalar_value_range_min_boundary() {
    let range = regex_syntax::hir::ClassUnicodeRange::new('\u{0000}', '\u{007F}');
    assert_eq!(range.start(), '\u{0000}');
    assert_eq!(range.end(), '\u{007F}');
}

