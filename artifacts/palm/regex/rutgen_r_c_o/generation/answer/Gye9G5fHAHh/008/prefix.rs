// Answer 0

#[test]
fn test_visit_class_set_binary_op_post_case_insensitive_difference() {
    let mut translator = Translator {
        stack: RefCell::new(vec![]),
        flags: Cell::new(Flags {
            unicode: Some(false),
            case_insensitive: Some(true),
            ..Default::default()
        }),
        allow_invalid_utf8: false,
    };

    let op = ast::ClassSetBinaryOp {
        span: Span::default(),
        kind: ast::ClassSetBinaryOpKind::Difference,
        lhs: Box::new(ClassSet::new(vec![])),
        rhs: Box::new(ClassSet::new(vec![])),
    };

    let mut lhs_class = ClassBytes::new(vec![]);
    let mut rhs_class = ClassBytes::new(vec![]);
    let mut cls_class = ClassBytes::new(vec![]);

    translator.push(HirFrame::ClassBytes(cls_class.clone()));
    translator.push(HirFrame::ClassBytes(lhs_class.clone()));
    translator.push(HirFrame::ClassBytes(rhs_class.clone()));

    translator.visit_class_set_binary_op_post(&op).ok();
}

