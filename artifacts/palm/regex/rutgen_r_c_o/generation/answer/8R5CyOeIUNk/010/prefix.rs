// Answer 0

#[test]
fn test_induct_class_bracketed_empty() {
    let mut visitor = MockVisitor::new();
    let ast = Ast::Class(Class::Bracketed(ClassBracketed {
        span: Span::new(0, 0),
        negated: false,
        kind: ClassSet::Normal(vec![]),
    }));
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.induct(&ast, &mut visitor);
}

#[test]
fn test_induct_class_bracketed_single() {
    let mut visitor = MockVisitor::new();
    let ast = Ast::Class(Class::Bracketed(ClassBracketed {
        span: Span::new(0, 1),
        negated: false,
        kind: ClassSet::Normal(vec![ClassSetItem::Literal('a')]),
    }));
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.induct(&ast, &mut visitor);
}

#[test]
fn test_induct_class_bracketed_multiple() {
    let mut visitor = MockVisitor::new();
    let ast = Ast::Class(Class::Bracketed(ClassBracketed {
        span: Span::new(0, 3),
        negated: false,
        kind: ClassSet::Normal(vec![
            ClassSetItem::Literal('a'),
            ClassSetItem::Literal('b'),
            ClassSetItem::Literal('c'),
        ]),
    }));
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.induct(&ast, &mut visitor);
}

#[test]
fn test_induct_repetition() {
    let mut visitor = MockVisitor::new();
    let ast = Ast::Repetition(Repetition {
        span: Span::new(0, 1),
        op: RepetitionOp::Plus,
        greedy: true,
        ast: Box::new(Ast::Literal(Literal::from('a'))),
    });
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.induct(&ast, &mut visitor);
}

#[test]
fn test_induct_group() {
    let mut visitor = MockVisitor::new();
    let ast = Ast::Group(Group {
        span: Span::new(0, 1),
        kind: GroupKind::Capture,
        ast: Box::new(Ast::Literal(Literal::from('a'))),
    });
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.induct(&ast, &mut visitor);
}

#[test]
fn test_induct_concat_non_empty() {
    let mut visitor = MockVisitor::new();
    let ast = Ast::Concat(Concat {
        span: Span::new(0, 2),
        asts: vec![
            Ast::Literal(Literal::from('a')),
            Ast::Literal(Literal::from('b')),
        ],
    });
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.induct(&ast, &mut visitor);
}

#[test]
fn test_induct_concat_empty() {
    let mut visitor = MockVisitor::new();
    let ast = Ast::Concat(Concat {
        span: Span::new(0, 0),
        asts: vec![],
    });
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.induct(&ast, &mut visitor);
}

#[test]
fn test_induct_alternation_non_empty() {
    let mut visitor = MockVisitor::new();
    let ast = Ast::Alternation(Alternation {
        span: Span::new(0, 2),
        asts: vec![
            Ast::Literal(Literal::from('a')),
            Ast::Literal(Literal::from('b')),
        ],
    });
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.induct(&ast, &mut visitor);
}

#[test]
fn test_induct_alternation_empty() {
    let mut visitor = MockVisitor::new();
    let ast = Ast::Alternation(Alternation {
        span: Span::new(0, 0),
        asts: vec![],
    });
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.induct(&ast, &mut visitor);
}

