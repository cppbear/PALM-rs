// Answer 0

#[test]
fn test_induct_alternation_empty() {
    let alternation = ast::Alternation {
        span: Span::new(0, 0),
        asts: Vec::new(),
    };
    let ast = Ast::Alternation(alternation);
    let mut visitor = MockVisitor::new();
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.induct(&ast, &mut visitor);
}

#[test]
fn test_induct_alternation_single() {
    let first_ast = Ast::Literal(Literal::from('a'));
    let alternation = ast::Alternation {
        span: Span::new(0, 1),
        asts: vec![first_ast],
    };
    let ast = Ast::Alternation(alternation);
    let mut visitor = MockVisitor::new();
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.induct(&ast, &mut visitor);
}

#[test]
fn test_induct_alternation_multiple() {
    let first_ast = Ast::Literal(Literal::from('a'));
    let second_ast = Ast::Literal(Literal::from('b'));
    let alternation = ast::Alternation {
        span: Span::new(0, 2),
        asts: vec![first_ast, second_ast],
    };
    let ast = Ast::Alternation(alternation);
    let mut visitor = MockVisitor::new();
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.induct(&ast, &mut visitor);
}

#[test]
fn test_induct_alternation_large() {
    let asts: Vec<Ast> = (0..10).map(|i| Ast::Literal(Literal::from((b'a' + i) as char))).collect();
    let alternation = ast::Alternation {
        span: Span::new(0, 10),
        asts,
    };
    let ast = Ast::Alternation(alternation);
    let mut visitor = MockVisitor::new();
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.induct(&ast, &mut visitor);
}

#[test]
fn test_induct_alternation_with_repetition() {
    let repetition_ast = Ast::Repetition(Repetition {
        span: Span::new(0, 3),
        op: RepetitionOp::Plus,
        greedy: true,
        ast: Box::new(Ast::Literal(Literal::from('a'))),
    });
    let alternation = ast::Alternation {
        span: Span::new(0, 4),
        asts: vec![repetition_ast],
    };
    let ast = Ast::Alternation(alternation);
    let mut visitor = MockVisitor::new();
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.induct(&ast, &mut visitor);
}

