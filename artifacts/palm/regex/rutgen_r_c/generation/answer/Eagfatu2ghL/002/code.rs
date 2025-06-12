// Answer 0

#[test]
fn test_hir_group_capture_name() {
    // Create a Translator instance
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags::default()),
        allow_invalid_utf8: false,
    };

    // Create a group with CaptureName kind
    let group = ast::Group {
        span: Span::default(),
        kind: ast::GroupKind::CaptureName {
            name: "test_capture".to_string(),
            index: 1,
        },
        ast: Box::new(Ast::default()),
    };

    // Create a Hir expression
    let expr = Hir::literal(hir::Literal::default());

    // Call the function under test
    let result_hir = TranslatorI::hir_group(&translator, &group, expr);

    // Check that the kind of the resulting Hir group is CaptureName
    if let HirKind::Group(ref hir_group) = result_hir.kind() {
        if let hir::GroupKind::CaptureName { name, index } = &hir_group.kind {
            assert_eq!(name, "test_capture");
            assert_eq!(*index, 1);
        } else {
            panic!("Expected GroupKind::CaptureName");
        }
    } else {
        panic!("Expected HirKind::Group");
    }
}

