// Answer 0

#[test]
fn test_empty_class_unicode() {
    use regex_syntax::hir::ClassUnicode;

    let class_unicode = ClassUnicode::new(vec![]);
    assert_eq!(class_unicode.ranges.len(), 0);
}

#[test]
fn test_empty_class_unicode_non_panic() {
    use regex_syntax::hir::ClassUnicode;

    let class_unicode = ClassUnicode::empty();
    assert_eq!(class_unicode.ranges.len(), 0);
}

