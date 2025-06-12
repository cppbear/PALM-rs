// Answer 0

#[test]
fn test_class_induct_item_empty() {
    let span = Span::new(0, 1);
    let item = ClassSetItem::Empty(span);
    let induct = ClassInduct::Item(&item);
    let mut output = String::new();
    let result = induct.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "Item(Empty)");
}

#[test]
fn test_class_induct_item_literal() {
    let span = Span::new(0, 1);
    let literal = Literal::new('a');
    let item = ClassSetItem::Literal(literal);
    let induct = ClassInduct::Item(&item);
    let mut output = String::new();
    let result = induct.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "Item(Literal)");
}

#[test]
fn test_class_induct_item_range() {
    let span = Span::new(0, 2);
    let range = ClassSetRange::new('a', 'z');
    let item = ClassSetItem::Range(range);
    let induct = ClassInduct::Item(&item);
    let mut output = String::new();
    let result = induct.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "Item(Range)");
}

#[test]
fn test_class_induct_item_ascii() {
    let ascii_class = ClassAscii::new("alnum");
    let item = ClassSetItem::Ascii(ascii_class);
    let induct = ClassInduct::Item(&item);
    let mut output = String::new();
    let result = induct.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "Item(Ascii)");
}

#[test]
fn test_class_induct_item_unicode() {
    let unicode_class = ClassUnicode::new("\\pL");
    let item = ClassSetItem::Unicode(unicode_class);
    let induct = ClassInduct::Item(&item);
    let mut output = String::new();
    let result = induct.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "Item(Unicode)");
}

#[test]
fn test_class_induct_item_perl() {
    let perl_class = ClassPerl::new("\\d");
    let item = ClassSetItem::Perl(perl_class);
    let induct = ClassInduct::Item(&item);
    let mut output = String::new();
    let result = induct.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "Item(Perl)");
}

#[test]
fn test_class_induct_item_bracketed() {
    let bracketed_class = ClassBracketed::new(vec![]);
    let item = ClassSetItem::Bracketed(Box::new(bracketed_class));
    let induct = ClassInduct::Item(&item);
    let mut output = String::new();
    let result = induct.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "Item(Bracketed)");
}

#[test]
fn test_class_induct_item_union() {
    let union_class = ClassSetUnion::new(vec![]);
    let item = ClassSetItem::Union(union_class);
    let induct = ClassInduct::Item(&item);
    let mut output = String::new();
    let result = induct.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "Item(Union)");
}

#[test]
fn test_class_induct_binary_op_intersection() {
    let lhs = ClassSet::new();
    let rhs = ClassSet::new();
    let binary_op = ClassSetBinaryOp {
        span: Span::new(0, 2),
        kind: ClassSetBinaryOpKind::Intersection,
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    };
    let induct = ClassInduct::BinaryOp(&binary_op);
    let mut output = String::new();
    let result = induct.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "BinaryOp(Intersection)");
}

#[test]
fn test_class_induct_binary_op_difference() {
    let lhs = ClassSet::new();
    let rhs = ClassSet::new();
    let binary_op = ClassSetBinaryOp {
        span: Span::new(0, 2),
        kind: ClassSetBinaryOpKind::Difference,
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    };
    let induct = ClassInduct::BinaryOp(&binary_op);
    let mut output = String::new();
    let result = induct.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "BinaryOp(Difference)");
}

#[test]
fn test_class_induct_binary_op_symmetric_difference() {
    let lhs = ClassSet::new();
    let rhs = ClassSet::new();
    let binary_op = ClassSetBinaryOp {
        span: Span::new(0, 2),
        kind: ClassSetBinaryOpKind::SymmetricDifference,
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    };
    let induct = ClassInduct::BinaryOp(&binary_op);
    let mut output = String::new();
    let result = induct.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "BinaryOp(SymmetricDifference)");
}

