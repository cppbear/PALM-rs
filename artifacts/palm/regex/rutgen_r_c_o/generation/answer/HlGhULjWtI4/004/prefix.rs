// Answer 0

#[test]
fn test_class_induct_item_empty() {
    let item = ast::ClassSetItem::Empty(Span::new(1, 5));
    let induct = ClassInduct::Item(&item);
    let mut buf = String::new();
    let _ = fmt(&induct, &mut buf);
}

#[test]
fn test_class_induct_item_literal() {
    let item = ast::ClassSetItem::Literal(Literal::from('a'));
    let induct = ClassInduct::Item(&item);
    let mut buf = String::new();
    let _ = fmt(&induct, &mut buf);
}

#[test]
fn test_class_induct_item_range() {
    let item = ast::ClassSetItem::Range(ClassSetRange::new('a', 'z'));
    let induct = ClassInduct::Item(&item);
    let mut buf = String::new();
    let _ = fmt(&induct, &mut buf);
}

#[test]
fn test_class_induct_item_asci() {
    let item = ast::ClassSetItem::Ascii(ClassAscii::new("alnum"));
    let induct = ClassInduct::Item(&item);
    let mut buf = String::new();
    let _ = fmt(&induct, &mut buf);
}

#[test]
fn test_class_induct_item_unicode() {
    let item = ast::ClassSetItem::Unicode(ClassUnicode::new("\\pL"));
    let induct = ClassInduct::Item(&item);
    let mut buf = String::new();
    let _ = fmt(&induct, &mut buf);
}

#[test]
fn test_class_induct_item_perl() {
    let item = ast::ClassSetItem::Perl(ClassPerl::new("\\d"));
    let induct = ClassInduct::Item(&item);
    let mut buf = String::new();
    let _ = fmt(&induct, &mut buf);
}

#[test]
fn test_class_induct_item_bracketed() {
    let item = ast::ClassSetItem::Bracketed(Box::new(ClassBracketed::new(vec![])));
    let induct = ClassInduct::Item(&item);
    let mut buf = String::new();
    let _ = fmt(&induct, &mut buf);
}

#[test]
fn test_class_induct_item_union() {
    let item = ast::ClassSetItem::Union(ClassSetUnion::new(vec![]));
    let induct = ClassInduct::Item(&item);
    let mut buf = String::new();
    let _ = fmt(&induct, &mut buf);
}

#[test]
fn test_class_induct_binary_op_intersection() {
    let lhs = ClassSet::new();
    let rhs = ClassSet::new();
    let op = ClassSetBinaryOp {
        span: Span::new(1, 5),
        kind: ClassSetBinaryOpKind::Intersection,
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    };
    let induct = ClassInduct::BinaryOp(&op);
    let mut buf = String::new();
    let _ = fmt(&induct, &mut buf);
}

#[test]
fn test_class_induct_binary_op_difference() {
    let lhs = ClassSet::new();
    let rhs = ClassSet::new();
    let op = ClassSetBinaryOp {
        span: Span::new(5, 10),
        kind: ClassSetBinaryOpKind::Difference,
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    };
    let induct = ClassInduct::BinaryOp(&op);
    let mut buf = String::new();
    let _ = fmt(&induct, &mut buf);
}

#[test]
fn test_class_induct_binary_op_symmetric_difference() {
    let lhs = ClassSet::new();
    let rhs = ClassSet::new();
    let op = ClassSetBinaryOp {
        span: Span::new(10, 20),
        kind: ClassSetBinaryOpKind::SymmetricDifference,
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    };
    let induct = ClassInduct::BinaryOp(&op);
    let mut buf = String::new();
    let _ = fmt(&induct, &mut buf);
}

