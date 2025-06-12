// Answer 0

#[test]
fn test_hir_group_capture_index() {
    struct MockAstGroup {
        kind: ast::GroupKind,
    }

    let group = MockAstGroup {
        kind: ast::GroupKind::CaptureIndex(1),
    };

    let expr_hir = Hir::empty();

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };

    let result = translator.hir_group(&group, expr_hir);
    assert_eq!(result.kind(), &HirKind::Group(hir::Group {
        kind: hir::GroupKind::CaptureIndex(1),
        hir: Box::new(Hir::empty()),
    }));
}

#[test]
fn test_hir_group_capture_name() {
    struct MockAstGroup {
        kind: ast::GroupKind,
    }

    let group = MockAstGroup {
        kind: ast::GroupKind::CaptureName {
            name: "test".to_string(),
            index: 2,
        },
    };

    let expr_hir = Hir::empty();

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };

    let result = translator.hir_group(&group, expr_hir);
    assert_eq!(result.kind(), &HirKind::Group(hir::Group {
        kind: hir::GroupKind::CaptureName {
            name: "test".to_string(),
            index: 2,
        },
        hir: Box::new(Hir::empty()),
    }));
}

#[test]
fn test_hir_group_non_capturing() {
    struct MockAstGroup {
        kind: ast::GroupKind,
    }

    let group = MockAstGroup {
        kind: ast::GroupKind::NonCapturing(true),
    };

    let expr_hir = Hir::empty();

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };

    let result = translator.hir_group(&group, expr_hir);
    assert_eq!(result.kind(), &HirKind::Group(hir::Group {
        kind: hir::GroupKind::NonCapturing,
        hir: Box::new(Hir::empty()),
    }));
}

