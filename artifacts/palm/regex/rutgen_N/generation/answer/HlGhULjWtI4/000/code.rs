// Answer 0

#[test]
fn test_fmt_class_induct_item_empty() {
    struct TestClassSetItem {
        item: ast::ClassSetItem,
    }

    impl fmt::Display for TestClassSetItem {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self.item {
                ast::ClassSetItem::Empty(_) => write!(f, "Item(Empty)"),
                _ => Ok(()),
            }
        }
    }

    enum ClassInduct {
        Item(TestClassSetItem),
        BinaryOp(TestBinaryOp),
    }

    struct TestBinaryOp {
        kind: ast::ClassSetBinaryOpKind,
    }

    let class_induct = ClassInduct::Item(TestClassSetItem { item: ast::ClassSetItem::Empty(()) });
    let result = format!("{}", class_induct);
    assert_eq!(result, "Item(Empty)");
}

#[test]
fn test_fmt_class_induct_item_literal() {
    struct TestClassSetItem {
        item: ast::ClassSetItem,
    }

    impl fmt::Display for TestClassSetItem {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self.item {
                ast::ClassSetItem::Literal(_) => write!(f, "Item(Literal)"),
                _ => Ok(()),
            }
        }
    }

    enum ClassInduct {
        Item(TestClassSetItem),
        BinaryOp(TestBinaryOp),
    }

    struct TestBinaryOp {
        kind: ast::ClassSetBinaryOpKind,
    }

    let class_induct = ClassInduct::Item(TestClassSetItem { item: ast::ClassSetItem::Literal(()) });
    let result = format!("{}", class_induct);
    assert_eq!(result, "Item(Literal)");
}

#[test]
fn test_fmt_class_induct_binary_op_intersection() {
    struct TestBinaryOp {
        kind: ast::ClassSetBinaryOpKind,
    }

    enum ClassInduct {
        Item(TestClassSetItem),
        BinaryOp(TestBinaryOp),
    }

    struct TestClassSetItem {
        item: ast::ClassSetItem,
    }

    impl fmt::Display for TestBinaryOp {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self.kind {
                ast::ClassSetBinaryOpKind::Intersection => write!(f, "BinaryOp(Intersection)"),
                _ => Ok(()),
            }
        }
    }

    let binary_op = TestBinaryOp { kind: ast::ClassSetBinaryOpKind::Intersection };
    let class_induct = ClassInduct::BinaryOp(binary_op);
    let result = format!("{}", class_induct);
    assert_eq!(result, "BinaryOp(Intersection)");
}

