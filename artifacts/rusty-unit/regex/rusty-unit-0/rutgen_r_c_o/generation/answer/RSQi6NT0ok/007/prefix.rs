// Answer 0

#[test]
fn test_induct_class_union_empty() {
    struct DummyVisitor;
    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();
    }
    let frame = ast::ClassSetUnion { span: Span::default(), items: vec![] };
    let item = ast::ClassSetItem::Union(frame);
    let induct = ClassInduct::Item(&item);
    let visitor = HeapVisitor::new();
    visitor.induct_class(&induct);
}

#[test]
fn test_induct_class_bracketed_item() {
    struct DummyVisitor;
    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();
    }
    let item = ClassBracketed {
        span: Span::default(),
        negated: false,
        kind: ClassSet::Item(ClassSetItem::Literal(Literal::from('a'))),
    };
    let class_set_item = ast::ClassSetItem::Bracketed(Box::new(item));
    let induct = ClassInduct::Item(&class_set_item);
    let visitor = HeapVisitor::new();
    visitor.induct_class(&induct);
}

#[test]
fn test_induct_class_bracketed_binary_op() {
    struct DummyVisitor;
    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();
    }
    let lhs = Box::new(ClassSet::Item(ClassSetItem::Literal(Literal::from('a'))));
    let rhs = Box::new(ClassSet::Item(ClassSetItem::Literal(Literal::from('b'))));
    let op = ClassSetBinaryOp {
        span: Span::default(),
        kind: ClassSetBinaryOpKind::Union,
        lhs,
        rhs,
    };
    let bracketed = ClassBracketed {
        span: Span::default(),
        negated: false,
        kind: ClassSet::BinaryOp(op),
    };
    let class_set_item = ast::ClassSetItem::Bracketed(Box::new(bracketed));
    let induct = ClassInduct::Item(&class_set_item);
    let visitor = HeapVisitor::new();
    visitor.induct_class(&induct);
}

#[test]
fn test_induct_class_union_non_empty() {
    struct DummyVisitor;
    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();
    }
    let item1 = ast::ClassSetItem::Literal(Literal::from('a'));
    let item2 = ast::ClassSetItem::Literal(Literal::from('b'));
    let union = ClassSetUnion {
        span: Span::default(),
        items: vec![item1, item2],
    };
    let class_set_item = ast::ClassSetItem::Union(union);
    let induct = ClassInduct::Item(&class_set_item);
    let visitor = HeapVisitor::new();
    visitor.induct_class(&induct);
}

