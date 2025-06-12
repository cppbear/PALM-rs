// Answer 0

#[test]
fn test_induct_class_union_non_empty() {
    struct DummyVisitor;
    struct DummyAst;

    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();
    }

    let span = Span::default();
    let item1 = ClassSetItem::Literal(Literal::new('a'));
    let item2 = ClassSetItem::Literal(Literal::new('b'));
    let union = ClassSetUnion {
        span,
        items: vec![item1.clone(), item2.clone()],
    };
    let class_set_item = ClassSetItem::Union(union);
    let class_induct = ClassInduct::Item(&class_set_item);
    let mut visitor = DummyVisitor;

    let visitor = HeapVisitor::new();
    let result = visitor.induct_class(&class_induct);

    assert!(result.is_some());
    if let Some(ClassFrame::Union { head, tail }) = result {
        assert_eq!(head, &item1);
        assert_eq!(tail, &vec![item2]);
    }
}

#[test]
fn test_induct_class_bracketed_item() {
    struct DummyVisitor;
    struct DummyAst;

    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();
    }

    let span = Span::default();
    let bracketed = ClassBracketed {
        span,
        negated: false,
        kind: ClassSet::Item(ClassSetItem::Literal(Literal::new('a'))),
    };
    let class_set_item = ClassSetItem::Bracketed(Box::new(bracketed));
    let class_induct = ClassInduct::Item(&class_set_item);
    let mut visitor = DummyVisitor;

    let visitor = HeapVisitor::new();
    let result = visitor.induct_class(&class_induct);

    assert!(result.is_some());
    if let Some(ClassFrame::Union { head, tail }) = result {
        assert_eq!(head, &ClassSetItem::Literal(Literal::new('a')));
        assert!(tail.is_empty());
    }
}

#[test]
fn test_induct_class_union_empty() {
    struct DummyVisitor;
    struct DummyAst;

    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();
    }

    let span = Span::default();
    let union = ClassSetUnion {
        span,
        items: Vec::new(),
    };
    let class_set_item = ClassSetItem::Union(union);
    let class_induct = ClassInduct::Item(&class_set_item);
    let mut visitor = DummyVisitor;

    let visitor = HeapVisitor::new();
    let result = visitor.induct_class(&class_induct);

    assert!(result.is_none());
}

