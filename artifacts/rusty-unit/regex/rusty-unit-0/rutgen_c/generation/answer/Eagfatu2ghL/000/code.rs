// Answer 0

#[test]
fn test_hir_group_capture_index() {
    struct DummyAstGroup {
        kind: ast::GroupKind,
    }
    
    let ast_group = DummyAstGroup {
        kind: ast::GroupKind::CaptureIndex(1),
    };

    let expr = Hir::literal(ast::Literal::new("test"));
    let translator = Translator {};
    let translator_instance = TranslatorI::new(&translator, "test pattern");
    
    let result = translator_instance.hir_group(&ast_group, expr);
    
    assert_eq!(result.kind(), &HirKind::Group(hir::Group {
        kind: hir::GroupKind::CaptureIndex(1),
        hir: Box::new(expr),
    }));
}

#[test]
fn test_hir_group_capture_name() {
    struct DummyAstGroup {
        kind: ast::GroupKind,
    }
    
    let ast_group = DummyAstGroup {
        kind: ast::GroupKind::CaptureName {
            name: String::from("test_name"),
            index: 2,
        },
    };

    let expr = Hir::literal(ast::Literal::new("test"));
    let translator = Translator {};
    let translator_instance = TranslatorI::new(&translator, "test pattern");

    let result = translator_instance.hir_group(&ast_group, expr);
    
    assert_eq!(result.kind(), &HirKind::Group(hir::Group {
        kind: hir::GroupKind::CaptureName {
            name: String::from("test_name"),
            index: 2,
        },
        hir: Box::new(expr),
    }));
}

#[test]
fn test_hir_group_non_capturing() {
    struct DummyAstGroup {
        kind: ast::GroupKind,
    }

    let ast_group = DummyAstGroup {
        kind: ast::GroupKind::NonCapturing(false),
    };

    let expr = Hir::literal(ast::Literal::new("test"));
    let translator = Translator {};
    let translator_instance = TranslatorI::new(&translator, "test pattern");

    let result = translator_instance.hir_group(&ast_group, expr);
    
    assert_eq!(result.kind(), &HirKind::Group(hir::Group {
        kind: hir::GroupKind::NonCapturing,
        hir: Box::new(expr),
    }));
}

