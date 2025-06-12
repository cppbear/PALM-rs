// Answer 0

#[test]
fn test_visit_class_set_binary_op_post_case_insensitive_difference_unicode() {
    let trans = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(Flags {
            case_insensitive: Some(true),
            multi_line: None,
            dot_matches_new_line: None,
            swap_greed: None,
            unicode: Some(true),
        }),
        allow_invalid_utf8: false,
    };

    let pattern = "test_pattern";
    let mut translator = TranslatorI::new(&trans, pattern);

    let cls = ClassUnicode::new(vec![]);
    let lhs = ClassUnicode::new(vec![]);
    let rhs = ClassUnicode::new(vec![]);

    translator.push(HirFrame::ClassUnicode(cls.clone()));
    translator.push(HirFrame::ClassUnicode(lhs.clone()));
    translator.push(HirFrame::ClassUnicode(rhs.clone()));

    let op = ast::ClassSetBinaryOp {
        span: Span::default(),
        kind: ast::ClassSetBinaryOpKind::Difference,
        lhs: Box::new(ast::ClassSet::new()),
        rhs: Box::new(ast::ClassSet::new()),
    };

    let _ = translator.visit_class_set_binary_op_post(&op);
}

#[test]
fn test_visit_class_set_binary_op_post_case_sensitive_difference_unicode() {
    let trans = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(Flags {
            case_insensitive: Some(false),
            multi_line: None,
            dot_matches_new_line: None,
            swap_greed: None,
            unicode: Some(true),
        }),
        allow_invalid_utf8: false,
    };

    let pattern = "test_pattern";
    let mut translator = TranslatorI::new(&trans, pattern);

    let cls = ClassUnicode::new(vec![]);
    let lhs = ClassUnicode::new(vec![]);
    let rhs = ClassUnicode::new(vec![]);

    translator.push(HirFrame::ClassUnicode(cls.clone()));
    translator.push(HirFrame::ClassUnicode(lhs.clone()));
    translator.push(HirFrame::ClassUnicode(rhs.clone()));

    let op = ast::ClassSetBinaryOp {
        span: Span::default(),
        kind: ast::ClassSetBinaryOpKind::Difference,
        lhs: Box::new(ast::ClassSet::new()),
        rhs: Box::new(ast::ClassSet::new()),
    };

    let _ = translator.visit_class_set_binary_op_post(&op);
}

