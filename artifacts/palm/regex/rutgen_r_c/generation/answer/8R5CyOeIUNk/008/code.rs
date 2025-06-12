// Answer 0

#[test]
fn test_induct_group() {
    struct DummyVisitor;

    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();
    }

    let ast = ast::Ast::Group(ast::Group {
        span: Span::default(),
        kind: GroupKind::default(),
        ast: Box::new(ast::Ast::Empty(Span::default())),
    });

    let mut visitor = DummyVisitor;
    let mut heap_visitor = HeapVisitor::new();

    let result = heap_visitor.induct(&ast, &mut visitor);
    assert!(result.is_ok());
    assert!(matches!(result.unwrap(), Some(Frame::Group(_))));
}

#[test]
fn test_induct_repetition() {
    struct DummyVisitor;

    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();
    }

    let ast = ast::Ast::Repetition(ast::Repetition {
        span: Span::default(),
        op: RepetitionOp::default(),
        greedy: true,
        ast: Box::new(ast::Ast::Empty(Span::default())),
    });

    let mut visitor = DummyVisitor;
    let mut heap_visitor = HeapVisitor::new();

    let result = heap_visitor.induct(&ast, &mut visitor);
    assert!(result.is_ok());
    assert!(matches!(result.unwrap(), Some(Frame::Repetition(_))));
}

#[test]
fn test_induct_concat_empty() {
    struct DummyVisitor;

    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();
    }

    let ast = ast::Ast::Concat(ast::Concat {
        span: Span::default(),
        asts: vec![],
    });

    let mut visitor = DummyVisitor;
    let mut heap_visitor = HeapVisitor::new();

    let result = heap_visitor.induct(&ast, &mut visitor);
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[test]
fn test_induct_concat_non_empty() {
    struct DummyVisitor;

    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();
    }

    let ast = ast::Ast::Concat(ast::Concat {
        span: Span::default(),
        asts: vec![
            ast::Ast::Empty(Span::default()),
            ast::Ast::Empty(Span::default()),
        ],
    });

    let mut visitor = DummyVisitor;
    let mut heap_visitor = HeapVisitor::new();

    let result = heap_visitor.induct(&ast, &mut visitor);
    assert!(result.is_ok());
    if let Some(Frame::Concat { head, tail }) = result.unwrap() {
        assert_eq!(head, &ast.asts[0]);
        assert_eq!(tail, &ast.asts[1..]);
    } else {
        panic!("Expected a Frame::Concat result");
    }
}

#[test]
fn test_induct_alternation_empty() {
    struct DummyVisitor;

    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();
    }

    let ast = ast::Ast::Alternation(ast::Alternation {
        span: Span::default(),
        asts: vec![],
    });

    let mut visitor = DummyVisitor;
    let mut heap_visitor = HeapVisitor::new();

    let result = heap_visitor.induct(&ast, &mut visitor);
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[test]
fn test_induct_alternation_non_empty() {
    struct DummyVisitor;

    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();
    }

    let ast = ast::Ast::Alternation(ast::Alternation {
        span: Span::default(),
        asts: vec![
            ast::Ast::Empty(Span::default()),
            ast::Ast::Empty(Span::default()),
        ],
    });

    let mut visitor = DummyVisitor;
    let mut heap_visitor = HeapVisitor::new();

    let result = heap_visitor.induct(&ast, &mut visitor);
    assert!(result.is_ok());
    if let Some(Frame::Alternation { head, tail }) = result.unwrap() {
        assert_eq!(head, &ast.asts[0]);
        assert_eq!(tail, &ast.asts[1..]);
    } else {
        panic!("Expected a Frame::Alternation result");
    }
}

