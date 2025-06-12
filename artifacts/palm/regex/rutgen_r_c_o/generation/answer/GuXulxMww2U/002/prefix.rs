// Answer 0

#[test]
fn test_visit_class_set_binary_op_in_non_unicode() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(false),
            ..Default::default()
        }),
        allow_invalid_utf8: true,
    };

    let pattern = "test_pattern";
    let mut translator_i = TranslatorI::new(&trans, pattern);

    let class_set_binary_op = ast::ClassSetBinaryOp {
        span: Span::new(0, 1),
        kind: ClassSetBinaryOpKind::Union,
        lhs: Box::new(ast::ClassSet::new()),
        rhs: Box::new(ast::ClassSet::new()),
    };

    let result = translator_i.visit_class_set_binary_op_in(&class_set_binary_op);
}

#[test]
fn test_visit_class_set_binary_op_in_non_unicode_with_different_flags() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(false),
            case_insensitive: Some(true),
            ..Default::default()
        }),
        allow_invalid_utf8: true,
    };

    let pattern = "another_test_pattern";
    let mut translator_i = TranslatorI::new(&trans, pattern);

    let class_set_binary_op = ast::ClassSetBinaryOp {
        span: Span::new(1, 2),
        kind: ClassSetBinaryOpKind::Intersect,
        lhs: Box::new(ast::ClassSet::new()),
        rhs: Box::new(ast::ClassSet::new()),
    };

    let result = translator_i.visit_class_set_binary_op_in(&class_set_binary_op);
}

#[test]
fn test_visit_class_set_binary_op_in_empty_binary_op() {
    let trans = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(false),
            ..Default::default()
        }),
        allow_invalid_utf8: true,
    };

    let pattern = "empty_op_pattern";
    let mut translator_i = TranslatorI::new(&trans, pattern);

    let class_set_binary_op = ast::ClassSetBinaryOp {
        span: Span::new(0, 0),
        kind: ClassSetBinaryOpKind::Union,
        lhs: Box::new(ast::ClassSet::new()),
        rhs: Box::new(ast::ClassSet::new()),
    };

    let result = translator_i.visit_class_set_binary_op_in(&class_set_binary_op);
}

