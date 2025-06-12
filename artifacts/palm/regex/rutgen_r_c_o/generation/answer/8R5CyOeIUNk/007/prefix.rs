// Answer 0

#[test]
fn test_induct_repetition() {
    let span = Span { start: 0, end: 5 };
    let repetition = Repetition {
        span,
        op: RepetitionOp::Plus,
        greedy: true,
        ast: Box::new(Ast::Literal(Literal::Char('a'))),
    };
    let ast = Ast::Repetition(repetition);
    
    let mut visitor = DummyVisitor {};
    let mut heap_visitor = HeapVisitor::new();
    let result = heap_visitor.induct(&ast, &mut visitor);
}

#[test]
fn test_induct_class_bracketed() {
    let span = Span { start: 0, end: 10 };
    let class_set = ClassBracketed {
        span,
        negated: false,
        kind: ClassSet::Union(vec![ClassSetItem::Literal('a'), ClassSetItem::Literal('b')]),
    };
    let ast = Ast::Class(Class::Bracketed(class_set));

    let mut visitor = DummyVisitor {};
    let mut heap_visitor = HeapVisitor::new();
    let result = heap_visitor.induct(&ast, &mut visitor);
}

#[test]
fn test_induct_group() {
    let span = Span { start: 0, end: 3 };
    let group = Group {
        span,
        kind: GroupKind::Capture,
        ast: Box::new(Ast::Literal(Literal::Char('a'))),
    };
    let ast = Ast::Group(group);

    let mut visitor = DummyVisitor {};
    let mut heap_visitor = HeapVisitor::new();
    let result = heap_visitor.induct(&ast, &mut visitor);
}

#[test]
fn test_induct_concat_non_empty() {
    let span1 = Span { start: 0, end: 1 };
    let lit1 = Ast::Literal(Literal::Char('a'));
    let span2 = Span { start: 1, end: 2 };
    let lit2 = Ast::Literal(Literal::Char('b'));

    let concat = Concat {
        span: Span { start: 0, end: 2 },
        asts: vec![lit1, lit2],
    };
    let ast = Ast::Concat(concat);

    let mut visitor = DummyVisitor {};
    let mut heap_visitor = HeapVisitor::new();
    let result = heap_visitor.induct(&ast, &mut visitor);
}

#[test]
fn test_induct_alternation_non_empty() {
    let span1 = Span { start: 0, end: 1 };
    let lit1 = Ast::Literal(Literal::Char('a'));
    let span2 = Span { start: 1, end: 2 };
    let lit2 = Ast::Literal(Literal::Char('b'));

    let alternation = Alternation {
        span: Span { start: 0, end: 2 },
        asts: vec![lit1, lit2],
    };
    let ast = Ast::Alternation(alternation);

    let mut visitor = DummyVisitor {};
    let mut heap_visitor = HeapVisitor::new();
    let result = heap_visitor.induct(&ast, &mut visitor);
}

