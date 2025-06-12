// Answer 0

#[test]
fn test_hir_group_capture_index_0() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };
    
    let group = ast::Group {
        span: Span::default(),
        kind: ast::GroupKind::CaptureIndex(0),
        ast: Box::new(Ast::default()),
    };

    let expr = Hir::empty();

    let translator_i = TranslatorI::new(&trans, "test_pattern");
    translator_i.hir_group(&group, expr);
}

#[test]
fn test_hir_group_capture_index_500() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };

    let group = ast::Group {
        span: Span::default(),
        kind: ast::GroupKind::CaptureIndex(500),
        ast: Box::new(Ast::default()),
    };

    let expr = Hir::empty();

    let translator_i = TranslatorI::new(&trans, "test_pattern");
    translator_i.hir_group(&group, expr);
}

#[test]
fn test_hir_group_capture_index_1000() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };

    let group = ast::Group {
        span: Span::default(),
        kind: ast::GroupKind::CaptureIndex(1000),
        ast: Box::new(Ast::default()),
    };

    let expr = Hir::empty();

    let translator_i = TranslatorI::new(&trans, "test_pattern");
    translator_i.hir_group(&group, expr);
}

