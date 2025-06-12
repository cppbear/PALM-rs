// Answer 0

#[test]
fn test_induct_concat_non_empty() {
    struct DummyVisitor;
    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();
    }

    let ast = Ast::Concat(Concat {
        span: Span { start: 0, end: 10 },
        asts: vec![
            Ast::Literal(Literal { value: "a" }),
            Ast::Literal(Literal { value: "b" }),
        ],
    });

    let mut visitor = DummyVisitor;
    let mut heap_visitor = HeapVisitor::new();

    let result = heap_visitor.induct(&ast, &mut visitor);
}

#[test]
fn test_induct_concat_multiple_nodes() {
    struct DummyVisitor;
    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();
    }

    let ast = Ast::Concat(Concat {
        span: Span { start: 0, end: 15 },
        asts: vec![
            Ast::Literal(Literal { value: "x" }),
            Ast::Literal(Literal { value: "y" }),
            Ast::Literal(Literal { value: "z" }),
        ],
    });

    let mut visitor = DummyVisitor;
    let mut heap_visitor = HeapVisitor::new();

    let result = heap_visitor.induct(&ast, &mut visitor);
}

#[test]
fn test_induct_concat_single_node() {
    struct DummyVisitor;
    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();
    }

    let ast = Ast::Concat(Concat {
        span: Span { start: 0, end: 5 },
        asts: vec![Ast::Literal(Literal { value: "c" })],
    });

    let mut visitor = DummyVisitor;
    let mut heap_visitor = HeapVisitor::new();

    let result = heap_visitor.induct(&ast, &mut visitor);
}

