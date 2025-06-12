// Answer 0

#[test]
fn test_class_induct_item_empty() {
    let item = ast::ClassSetItem::Empty(Span::default());
    let induct = ClassInduct::Item(&item);
    let mut output = String::new();
    induct.fmt(&mut output).unwrap();
    assert_eq!(output, "Item(Empty)");
}

#[test]
fn test_class_induct_item_literal() {
    let item = ast::ClassSetItem::Literal(Literal::default());
    let induct = ClassInduct::Item(&item);
    let mut output = String::new();
    induct.fmt(&mut output).unwrap();
    assert_eq!(output, "Item(Literal)");
}

#[test]
fn test_class_induct_item_range() {
    let item = ast::ClassSetItem::Range(ClassSetRange::default());
    let induct = ClassInduct::Item(&item);
    let mut output = String::new();
    induct.fmt(&mut output).unwrap();
    assert_eq!(output, "Item(Range)");
}

#[test]
fn test_class_induct_item_ascii() {
    let item = ast::ClassSetItem::Ascii(ClassAscii::default());
    let induct = ClassInduct::Item(&item);
    let mut output = String::new();
    induct.fmt(&mut output).unwrap();
    assert_eq!(output, "Item(Ascii)");
}

#[test]
fn test_class_induct_item_unicode() {
    let item = ast::ClassSetItem::Unicode(ClassUnicode::default());
    let induct = ClassInduct::Item(&item);
    let mut output = String::new();
    induct.fmt(&mut output).unwrap();
    assert_eq!(output, "Item(Unicode)");
}

#[test]
fn test_class_induct_item_perl() {
    let item = ast::ClassSetItem::Perl(ClassPerl::default());
    let induct = ClassInduct::Item(&item);
    let mut output = String::new();
    induct.fmt(&mut output).unwrap();
    assert_eq!(output, "Item(Perl)");
}

#[test]
fn test_class_induct_item_bracketed() {
    let item = ast::ClassSetItem::Bracketed(Box::new(ClassBracketed::default()));
    let induct = ClassInduct::Item(&item);
    let mut output = String::new();
    induct.fmt(&mut output).unwrap();
    assert_eq!(output, "Item(Bracketed)");
}

#[test]
fn test_class_induct_item_union() {
    let item = ast::ClassSetItem::Union(ClassSetUnion::default());
    let induct = ClassInduct::Item(&item);
    let mut output = String::new();
    induct.fmt(&mut output).unwrap();
    assert_eq!(output, "Item(Union)");
}

#[test]
fn test_class_induct_binary_op_intersection() {
    let op = ast::ClassSetBinaryOp {
        span: Span::default(),
        kind: ast::ClassSetBinaryOpKind::Intersection,
        lhs: Box::new(ClassSet::default()),
        rhs: Box::new(ClassSet::default()),
    };
    let induct = ClassInduct::BinaryOp(&op);
    let mut output = String::new();
    induct.fmt(&mut output).unwrap();
    assert_eq!(output, "BinaryOp(Intersection)");
}

#[test]
fn test_class_induct_binary_op_difference() {
    let op = ast::ClassSetBinaryOp {
        span: Span::default(),
        kind: ast::ClassSetBinaryOpKind::Difference,
        lhs: Box::new(ClassSet::default()),
        rhs: Box::new(ClassSet::default()),
    };
    let induct = ClassInduct::BinaryOp(&op);
    let mut output = String::new();
    induct.fmt(&mut output).unwrap();
    assert_eq!(output, "BinaryOp(Difference)");
}

#[test]
fn test_class_induct_binary_op_symmetric_difference() {
    let op = ast::ClassSetBinaryOp {
        span: Span::default(),
        kind: ast::ClassSetBinaryOpKind::SymmetricDifference,
        lhs: Box::new(ClassSet::default()),
        rhs: Box::new(ClassSet::default()),
    };
    let induct = ClassInduct::BinaryOp(&op);
    let mut output = String::new();
    induct.fmt(&mut output).unwrap();
    assert_eq!(output, "BinaryOp(SymmetricDifference)");
}

