// Answer 0

#[test]
fn test_induct_empty_concat() {
    struct TestVisitor;
    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
    }

    let mut visitor = TestVisitor;
    let ast = Ast::Concat(Concat {
        span: Span::default(),
        asts: Vec::new(),
    });
    let mut heap_visitor = HeapVisitor::new();
    let result = heap_visitor.induct(&ast, &mut visitor);
    assert_eq!(result.unwrap(), None);
}

#[test]
fn test_induct_non_empty_concat() {
    struct TestVisitor;
    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
    }

    let mut visitor = TestVisitor;
    let ast1 = Ast::Literal(Literal::default());
    let ast2 = Ast::Literal(Literal::default());
    let ast = Ast::Concat(Concat {
        span: Span::default(),
        asts: vec![ast1.clone(), ast2.clone()],
    });
    let mut heap_visitor = HeapVisitor::new();
    let result = heap_visitor.induct(&ast, &mut visitor).unwrap();
    if let Some(Frame::Concat { head, tail }) = result {
        assert_eq!(head, &ast1);
        assert_eq!(tail, &[ast2]);
    } else {
        panic!("Expected a concat frame");
    }
}

#[test]
fn test_induct_repetition() {
    struct TestVisitor;
    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
    }

    let mut visitor = TestVisitor;
    let ast = Ast::Repetition(Repetition {
        span: Span::default(),
        op: RepetitionOp::ZeroOrMore,
        greedy: true,
        ast: Box::new(Ast::Literal(Literal::default())),
    });
    let mut heap_visitor = HeapVisitor::new();
    let result = heap_visitor.induct(&ast, &mut visitor).unwrap();
    if let Some(Frame::Repetition(_)) = result {
        // Successfully created a Repetition frame
    } else {
        panic!("Expected a repetition frame");
    }
}

#[test]
fn test_induct_group() {
    struct TestVisitor;
    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
    }

    let mut visitor = TestVisitor;
    let ast = Ast::Group(Group {
        span: Span::default(),
        kind: GroupKind::Capture,
        ast: Box::new(Ast::Literal(Literal::default())),
    });
    let mut heap_visitor = HeapVisitor::new();
    let result = heap_visitor.induct(&ast, &mut visitor).unwrap();
    if let Some(Frame::Group(_)) = result {
        // Successfully created a Group frame
    } else {
        panic!("Expected a group frame");
    }
}

#[test]
fn test_induct_alternation_empty() {
    struct TestVisitor;
    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
    }

    let mut visitor = TestVisitor;
    let ast = Ast::Alternation(Alternation {
        span: Span::default(),
        asts: Vec::new(),
    });
    let mut heap_visitor = HeapVisitor::new();
    let result = heap_visitor.induct(&ast, &mut visitor);
    assert_eq!(result.unwrap(), None);
}

#[test]
fn test_induct_alternation_non_empty() {
    struct TestVisitor;
    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
    }

    let mut visitor = TestVisitor;
    let ast1 = Ast::Literal(Literal::default());
    let ast2 = Ast::Literal(Literal::default());
    let ast = Ast::Alternation(Alternation {
        span: Span::default(),
        asts: vec![ast1.clone(), ast2.clone()],
    });
    let mut heap_visitor = HeapVisitor::new();
    let result = heap_visitor.induct(&ast, &mut visitor).unwrap();
    if let Some(Frame::Alternation { head, tail }) = result {
        assert_eq!(head, &ast1);
        assert_eq!(tail, &[ast2]);
    } else {
        panic!("Expected an alternation frame");
    }
}

