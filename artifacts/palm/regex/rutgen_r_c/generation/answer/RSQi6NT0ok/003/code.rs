// Answer 0

#[test]
fn test_induct_class_with_empty_union() {
    struct DummyVisitor {}

    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();

        fn visit(&self, _node: &Ast) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    let ast = ClassInduct::Item(Box::new(ClassSetItem::Union(ClassSetUnion {
        span: Span::default(),
        items: vec![],
    })));

    let visitor = DummyVisitor {};
    let mut heap_visitor = HeapVisitor::new();

    let result = heap_visitor.induct_class(&ast);
    assert_eq!(result, None);
}

#[test]
fn test_induct_class_with_bracketed_class_item() {
    struct DummyVisitor {}

    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();

        fn visit(&self, _node: &Ast) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    let ast = ClassInduct::Item(Box::new(ClassSetItem::Bracketed(Box::new(ClassBracketed {
        span: Span::default(),
        negated: false,
        kind: ClassSet::BinaryOp(ClassSetBinaryOp {
            span: Span::default(),
            kind: ClassSetBinaryOpKind::And,
            lhs: Box::new(ClassSet::Item(ClassSetItem::Literal(Literal::new('a')))),
            rhs: Box::new(ClassSet::Item(ClassSetItem::Literal(Literal::new('b')))),
        }),
    }))));

    let visitor = DummyVisitor {};
    let mut heap_visitor = HeapVisitor::new();

    let result = heap_visitor.induct_class(&ast);
    assert!(result.is_some());
}

#[test]
fn test_induct_class_with_bare_item() {
    struct DummyVisitor {}

    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();

        fn visit(&self, _node: &Ast) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    let ast = ClassInduct::BinaryOp(ClassSetBinaryOp {
        span: Span::default(),
        kind: ClassSetBinaryOpKind::Subtraction,
        lhs: Box::new(ClassSet::Item(ClassSetItem::Bracketed(Box::new(ClassBracketed {
            span: Span::default(),
            negated: false,
            kind: ClassSet::Item(ClassSetItem::Literal(Literal::new('c'))),
        })))),
        rhs: Box::new(ClassSet::Item(ClassSetItem::Bracketed(Box::new(ClassBracketed {
            span: Span::default(),
            negated: true,
            kind: ClassSet::Item(ClassSetItem::Literal(Literal::new('d'))),
        }))))
    });

    let visitor = DummyVisitor {};
    let mut heap_visitor = HeapVisitor::new();

    let result = heap_visitor.induct_class(&ast);
    assert!(result.is_some());
}

