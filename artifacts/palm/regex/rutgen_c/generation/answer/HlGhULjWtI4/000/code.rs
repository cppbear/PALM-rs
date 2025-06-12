// Answer 0

#[test]
fn test_class_induct_item_empty() {
    struct Span;
    let item = ast::ClassSetItem::Empty(Span);
    let induct = ClassInduct::Item(&item);
    let mut buffer = Vec::new();
    let result = fmt::write(&mut buffer, induct);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(buffer).unwrap(), "Item(Empty)");
}

#[test]
fn test_class_induct_item_literal() {
    struct Span;
    struct Literal;
    let item = ast::ClassSetItem::Literal(Literal);
    let induct = ClassInduct::Item(&item);
    let mut buffer = Vec::new();
    let result = fmt::write(&mut buffer, induct);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(buffer).unwrap(), "Item(Literal)");
}

#[test]
fn test_class_induct_item_range() {
    struct Span;
    struct ClassSetRange;
    let item = ast::ClassSetItem::Range(ClassSetRange);
    let induct = ClassInduct::Item(&item);
    let mut buffer = Vec::new();
    let result = fmt::write(&mut buffer, induct);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(buffer).unwrap(), "Item(Range)");
}

#[test]
fn test_class_induct_item_ascii() {
    struct ClassAscii;
    let item = ast::ClassSetItem::Ascii(ClassAscii);
    let induct = ClassInduct::Item(&item);
    let mut buffer = Vec::new();
    let result = fmt::write(&mut buffer, induct);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(buffer).unwrap(), "Item(Ascii)");
}

#[test]
fn test_class_induct_item_unicode() {
    struct ClassUnicode;
    let item = ast::ClassSetItem::Unicode(ClassUnicode);
    let induct = ClassInduct::Item(&item);
    let mut buffer = Vec::new();
    let result = fmt::write(&mut buffer, induct);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(buffer).unwrap(), "Item(Unicode)");
}

#[test]
fn test_class_induct_item_perl() {
    struct ClassPerl;
    let item = ast::ClassSetItem::Perl(ClassPerl);
    let induct = ClassInduct::Item(&item);
    let mut buffer = Vec::new();
    let result = fmt::write(&mut buffer, induct);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(buffer).unwrap(), "Item(Perl)");
}

#[test]
fn test_class_induct_item_bracketed() {
    struct ClassBracketed;
    let item = ast::ClassSetItem::Bracketed(Box::new(ClassBracketed));
    let induct = ClassInduct::Item(&item);
    let mut buffer = Vec::new();
    let result = fmt::write(&mut buffer, induct);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(buffer).unwrap(), "Item(Bracketed)");
}

#[test]
fn test_class_induct_item_union() {
    struct ClassSetUnion;
    let item = ast::ClassSetItem::Union(ClassSetUnion);
    let induct = ClassInduct::Item(&item);
    let mut buffer = Vec::new();
    let result = fmt::write(&mut buffer, induct);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(buffer).unwrap(), "Item(Union)");
}

#[test]
fn test_class_induct_binary_op_intersection() {
    struct ClassSetBinaryOpKind {
        kind: ast::ClassSetBinaryOpKind,
    }
    let op = ClassSetBinaryOpKind { kind: ast::ClassSetBinaryOpKind::Intersection };
    let induct = ClassInduct::BinaryOp(&op);
    let mut buffer = Vec::new();
    let result = fmt::write(&mut buffer, induct);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(buffer).unwrap(), "BinaryOp(Intersection)");
}

#[test]
fn test_class_induct_binary_op_difference() {
    struct ClassSetBinaryOpKind {
        kind: ast::ClassSetBinaryOpKind,
    }
    let op = ClassSetBinaryOpKind { kind: ast::ClassSetBinaryOpKind::Difference };
    let induct = ClassInduct::BinaryOp(&op);
    let mut buffer = Vec::new();
    let result = fmt::write(&mut buffer, induct);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(buffer).unwrap(), "BinaryOp(Difference)");
}

#[test]
fn test_class_induct_binary_op_symmetric_difference() {
    struct ClassSetBinaryOpKind {
        kind: ast::ClassSetBinaryOpKind,
    }
    let op = ClassSetBinaryOpKind { kind: ast::ClassSetBinaryOpKind::SymmetricDifference };
    let induct = ClassInduct::BinaryOp(&op);
    let mut buffer = Vec::new();
    let result = fmt::write(&mut buffer, induct);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(buffer).unwrap(), "BinaryOp(SymmetricDifference)");
}

