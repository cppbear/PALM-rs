// Answer 0

#[test]
fn test_hir_group_non_capturing() {
    struct DummyAst;
    impl ast::Group {
        fn new_non_capturing() -> Self {
            ast::Group {
                kind: ast::GroupKind::NonCapturing(Box::new(DummyAst)),
                span: Span::default(),
                ast: Box::new(ast::Ast::default()),
            }
        }
    }

    struct DummyHir;
    impl Hir {
        fn new() -> Self {
            Hir::empty()
        }
    }

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };

    let group = DummyAst::new_non_capturing();
    let expr = DummyHir::new();

    let result = translator.hir_group(&group, expr);
    assert_eq!(result.kind(), &HirKind::Group(hir::Group {
        kind: hir::GroupKind::NonCapturing,
        hir: Box::new(expr),
    }));
}

#[test]
fn test_hir_group_capture_index() {
    struct DummyAst {
        index: u32,
    }

    impl DummyAst {
        fn new_capture_index(index: u32) -> ast::Group {
            ast::Group {
                kind: ast::GroupKind::CaptureIndex(index),
                span: Span::default(),
                ast: Box::new(ast::Ast::default()),
            }
        }
    }

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };

    let group = DummyAst::new_capture_index(1);
    let expr = Hir::empty();

    let result = translator.hir_group(&group, expr);
    assert_eq!(result.kind(), &HirKind::Group(hir::Group {
        kind: hir::GroupKind::CaptureIndex(1),
        hir: Box::new(expr),
    }));
}

#[test]
fn test_hir_group_capture_name() {
    struct DummyAst {
        name: String,
        index: u32,
    }

    impl DummyAst {
        fn new_capture_name(name: &str, index: u32) -> ast::Group {
            ast::Group {
                kind: ast::GroupKind::CaptureName {
                    name: name.to_string(),
                    index,
                },
                span: Span::default(),
                ast: Box::new(ast::Ast::default()),
            }
        }
    }

    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };

    let group = DummyAst::new_capture_name("group_name", 2);
    let expr = Hir::empty();

    let result = translator.hir_group(&group, expr);
    assert_eq!(result.kind(), &HirKind::Group(hir::Group {
        kind: hir::GroupKind::CaptureName {
            name: "group_name".to_string(),
            index: 2,
        },
        hir: Box::new(expr),
    }));
}

