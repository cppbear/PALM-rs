// Answer 0

#[test]
fn test_unwrap_class_unicode_valid() {
    let class_unicode = hir::ClassUnicode {
        span: Span::default(),
        negated: false,
        kind: ClassUnicodeKind::default(),
    };
    let frame = HirFrame::ClassUnicode(class_unicode.clone());
    let unwrapped = frame.unwrap_class_unicode();
    assert_eq!(unwrapped, class_unicode);
}

#[test]
#[should_panic(expected = "tried to unwrap Unicode class from HirFrame, got:")]
fn test_unwrap_class_unicode_invalid() {
    let frame = HirFrame::Expr(Hir::default());
    frame.unwrap_class_unicode();
}

