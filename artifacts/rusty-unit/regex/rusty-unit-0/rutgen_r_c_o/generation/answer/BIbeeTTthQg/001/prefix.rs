// Answer 0

#[test]
#[should_panic]
fn test_unwrap_expr_class_unicode() {
    let class_unicode = hir::ClassUnicode {
        span: Span::default(),
        negated: false,
        kind: ClassUnicodeKind::default(),
    };
    let frame = HirFrame::ClassUnicode(class_unicode);
    frame.unwrap_expr();
}

#[test]
#[should_panic]
fn test_unwrap_expr_class_bytes() {
    let class_bytes = hir::ClassBytes {
        set: IntervalSet::default(),
    };
    let frame = HirFrame::ClassBytes(class_bytes);
    frame.unwrap_expr();
}

#[test]
#[should_panic]
fn test_unwrap_expr_group() {
    let flags = Some(Flags::default());
    let frame = HirFrame::Group { old_flags: flags };
    frame.unwrap_expr();
}

#[test]
#[should_panic]
fn test_unwrap_expr_concat() {
    let frame = HirFrame::Concat;
    frame.unwrap_expr();
}

#[test]
#[should_panic]
fn test_unwrap_expr_alternation() {
    let frame = HirFrame::Alternation;
    frame.unwrap_expr();
}

