// Answer 0

#[test]
fn test_unwrap_class_unicode_valid() {
    #[derive(Clone, Debug, Eq, PartialEq)]
    struct DummyClassUnicode {
        span: ast::Span,
        negated: bool,
        kind: hir::ClassUnicodeKind,
    }

    let unicode_class = hir::ClassUnicode {
        span: ast::Span { start: 0, end: 1 },
        negated: false,
        kind: DummyClassUnicode {
            span: ast::Span { start: 0, end: 1 },
            negated: true,
            kind: hir::ClassUnicodeKind::Basic,
        },
    };

    let frame = HirFrame::ClassUnicode(unicode_class.clone());
    let result = frame.unwrap_class_unicode();
    
    assert_eq!(result, unicode_class);
}

#[should_panic(expected = "tried to unwrap Unicode class from HirFrame, got:")]
#[test]
fn test_unwrap_class_unicode_invalid() {
    let frame = HirFrame::Expr(Hir { kind: hir::HirKind::Empty, info: HirInfo::default() });
    
    frame.unwrap_class_unicode();
}

