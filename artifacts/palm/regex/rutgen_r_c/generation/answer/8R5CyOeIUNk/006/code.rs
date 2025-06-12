// Answer 0

#[test]
fn test_induct_with_group() {
    struct TestVisitor;

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        fn visit_group(&mut self, _group: &Group) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    let ast_group = Ast::Group(Group {
        span: Span::default(),
        kind: GroupKind::default(),
        ast: Box::new(Ast::Empty(Span::default())),
    });

    let mut visitor = TestVisitor;
    let mut heap_visitor = HeapVisitor::new();

    let result = heap_visitor.induct(&ast_group, &mut visitor);
    assert!(result.is_ok());
    assert!(matches!(result.unwrap(), Some(Frame::Group(_))));
}

#[test]
fn test_induct_with_concat() {
    struct TestVisitor;

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        fn visit_group(&mut self, _group: &Group) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    let ast_concat = Ast::Concat(Concat {
        span: Span::default(),
        asts: vec![
            Ast::Literal(Literal::default()),
            Ast::Group(Group {
                span: Span::default(),
                kind: GroupKind::default(),
                ast: Box::new(Ast::Empty(Span::default())),
            }),
        ],
    });

    let mut visitor = TestVisitor;
    let mut heap_visitor = HeapVisitor::new();

    let result = heap_visitor.induct(&ast_concat, &mut visitor);
    assert!(result.is_ok());
    assert!(matches!(result.unwrap(), Some(Frame::Concat { head: _, tail: _ })));
}

#[test]
fn test_induct_with_empty_concat() {
    struct TestVisitor;

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
    }

    let ast_concat = Ast::Concat(Concat {
        span: Span::default(),
        asts: vec![],
    });

    let mut visitor = TestVisitor;
    let mut heap_visitor = HeapVisitor::new();

    let result = heap_visitor.induct(&ast_concat, &mut visitor);
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[test]
fn test_induct_with_repetition() {
    struct TestVisitor;

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
    }

    let ast_repetition = Ast::Repetition(Repetition {
        span: Span::default(),
        op: RepetitionOp::default(),
        greedy: true,
        ast: Box::new(Ast::Group(Group {
            span: Span::default(),
            kind: GroupKind::default(),
            ast: Box::new(Ast::Empty(Span::default())),
        })),
    });

    let mut visitor = TestVisitor;
    let mut heap_visitor = HeapVisitor::new();

    let result = heap_visitor.induct(&ast_repetition, &mut visitor);
    assert!(result.is_ok());
    assert!(matches!(result.unwrap(), Some(Frame::Repetition(_))));
}

#[test]
fn test_induct_with_empty_alternation() {
    struct TestVisitor;

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
    }

    let ast_alternation = Ast::Alternation(Alternation {
        span: Span::default(),
        asts: vec![],
    });

    let mut visitor = TestVisitor;
    let mut heap_visitor = HeapVisitor::new();

    let result = heap_visitor.induct(&ast_alternation, &mut visitor);
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

