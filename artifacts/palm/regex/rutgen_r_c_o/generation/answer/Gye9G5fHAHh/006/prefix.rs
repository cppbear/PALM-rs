// Answer 0

#[test]
fn test_visit_class_set_binary_op_post_unicode_intersection() {
    let trans = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(Flags {
            unicode: Some(true),
            case_insensitive: Some(false),
            ..Default::default()
        }),
        allow_invalid_utf8: false,
    };

    let mut translator_i = TranslatorI::new(&trans, "test_pattern");

    let class_unicode_1 = ClassUnicode::new(vec![ClassUnicodeRange::new('a', 'z')]);
    let class_unicode_2 = ClassUnicode::new(vec![ClassUnicodeRange::new('c', 'f')]);
    let class_unicode_3 = ClassUnicode::new(vec![ClassUnicodeRange::new('g', 'm')]);

    translator_i.push(HirFrame::ClassUnicode(class_unicode_1));
    translator_i.push(HirFrame::ClassUnicode(class_unicode_2));
    translator_i.push(HirFrame::ClassUnicode(class_unicode_3));

    let operation = ast::ClassSetBinaryOp {
        span: Span::default(),
        kind: ast::ClassSetBinaryOpKind::Intersection,
        lhs: Box::new(ast::ClassSet::default()),
        rhs: Box::new(ast::ClassSet::default()),
    };

    translator_i.visit_class_set_binary_op_post(&operation).unwrap();
} 

#[test]
fn test_visit_class_set_binary_op_post_unicode_difference() {
    let trans = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(Flags {
            unicode: Some(true),
            case_insensitive: Some(false),
            ..Default::default()
        }),
        allow_invalid_utf8: false,
    };

    let mut translator_i = TranslatorI::new(&trans, "test_pattern");

    let class_unicode_1 = ClassUnicode::new(vec![ClassUnicodeRange::new('a', 'z')]);
    let class_unicode_2 = ClassUnicode::new(vec![ClassUnicodeRange::new('c', 'f')]);
    let class_unicode_3 = ClassUnicode::new(vec![ClassUnicodeRange::new('g', 'm')]);

    translator_i.push(HirFrame::ClassUnicode(class_unicode_1));
    translator_i.push(HirFrame::ClassUnicode(class_unicode_2));
    translator_i.push(HirFrame::ClassUnicode(class_unicode_3));

    let operation = ast::ClassSetBinaryOp {
        span: Span::default(),
        kind: ast::ClassSetBinaryOpKind::Difference,
        lhs: Box::new(ast::ClassSet::default()),
        rhs: Box::new(ast::ClassSet::default()),
    };

    translator_i.visit_class_set_binary_op_post(&operation).unwrap();
}

#[test]
fn test_visit_class_set_binary_op_post_unicode_symmetric_difference() {
    let trans = Translator {
        stack: RefCell::new(Vec::new()),
        flags: Cell::new(Flags {
            unicode: Some(true),
            case_insensitive: Some(false),
            ..Default::default()
        }),
        allow_invalid_utf8: false,
    };

    let mut translator_i = TranslatorI::new(&trans, "test_pattern");

    let class_unicode_1 = ClassUnicode::new(vec![ClassUnicodeRange::new('a', 'z')]);
    let class_unicode_2 = ClassUnicode::new(vec![ClassUnicodeRange::new('c', 'f')]);
    let class_unicode_3 = ClassUnicode::new(vec![ClassUnicodeRange::new('g', 'm')]);

    translator_i.push(HirFrame::ClassUnicode(class_unicode_1));
    translator_i.push(HirFrame::ClassUnicode(class_unicode_2));
    translator_i.push(HirFrame::ClassUnicode(class_unicode_3));

    let operation = ast::ClassSetBinaryOp {
        span: Span::default(),
        kind: ast::ClassSetBinaryOpKind::SymmetricDifference,
        lhs: Box::new(ast::ClassSet::default()),
        rhs: Box::new(ast::ClassSet::default()),
    };

    translator_i.visit_class_set_binary_op_post(&operation).unwrap();
}

