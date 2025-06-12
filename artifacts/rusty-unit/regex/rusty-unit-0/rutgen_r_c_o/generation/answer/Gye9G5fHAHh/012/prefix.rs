// Answer 0

#[test]
fn test_visit_class_set_binary_op_post_intersection_case_sensitive_bytes() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            case_insensitive: Some(false),
            unicode: Some(false),
            ..Flags::default()
        }),
        allow_invalid_utf8: false,
    };

    let class_bytes1 = ClassBytes::new(vec![ClassBytesRange::new(1, 5)]);
    let class_bytes2 = ClassBytes::new(vec![ClassBytesRange::new(3, 7)]);
    let class_bytes3 = ClassBytes::new(vec![ClassBytesRange::new(2, 6)]);
    
    let op = ast::ClassSetBinaryOp {
        span: Span::default(),
        kind: ast::ClassSetBinaryOpKind::Intersection,
        lhs: Box::new(class_bytes1),
        rhs: Box::new(class_bytes2),
    };
    
    let mut translator_i = TranslatorI::new(&translator, "test-pattern");
    translator_i.push(HirFrame::ClassBytes(class_bytes3));
    translator_i.push(HirFrame::ClassBytes(class_bytes1));
    translator_i.push(HirFrame::ClassBytes(class_bytes2));
    
    let _ = translator_i.visit_class_set_binary_op_post(&op);
}

#[test]
fn test_visit_class_set_binary_op_post_intersection_case_sensitive_bytes_edge() {
    let translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            case_insensitive: Some(false),
            unicode: Some(false),
            ..Flags::default()
        }),
        allow_invalid_utf8: false,
    };

    let class_bytes1 = ClassBytes::new(vec![ClassBytesRange::new(0, 0)]);
    let class_bytes2 = ClassBytes::new(vec![ClassBytesRange::new(0, 0)]);
    let class_bytes3 = ClassBytes::new(vec![ClassBytesRange::new(0, 0)]);

    let op = ast::ClassSetBinaryOp {
        span: Span::default(),
        kind: ast::ClassSetBinaryOpKind::Intersection,
        lhs: Box::new(class_bytes1),
        rhs: Box::new(class_bytes2),
    };

    let mut translator_i = TranslatorI::new(&translator, "test-pattern");
    translator_i.push(HirFrame::ClassBytes(class_bytes3));
    translator_i.push(HirFrame::ClassBytes(class_bytes1));
    translator_i.push(HirFrame::ClassBytes(class_bytes2));

    let _ = translator_i.visit_class_set_binary_op_post(&op);
}

