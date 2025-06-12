// Answer 0

#[test]
fn test_span_empty() {
    let span = Span { start: Position::new(0), end: Position::new(1) };
    let ast = Ast::Empty(span.clone());
    let result = ast.span();
}

#[test]
fn test_span_literal() {
    let span = Span { start: Position::new(1), end: Position::new(2) };
    let literal = Literal { span, kind: LiteralKind::Unicode('a'), c: 'a' };
    let ast = Ast::Literal(literal);
    let result = ast.span();
}

#[test]
fn test_span_dot() {
    let span = Span { start: Position::new(3), end: Position::new(4) };
    let ast = Ast::Dot(span.clone());
    let result = ast.span();
}

#[test]
fn test_span_assertion() {
    let span = Span { start: Position::new(5), end: Position::new(6) };
    let assertion = Assertion { span, kind: AssertionKind::WordBoundary };
    let ast = Ast::Assertion(assertion);
    let result = ast.span();
}

#[test]
fn test_span_class() {
    let span = Span { start: Position::new(7), end: Position::new(8) };
    let class = Class::Unicode(ClassUnicode { span });
    let ast = Ast::Class(class);
    let result = ast.span();
}

#[test]
fn test_span_repetition() {
    let span = Span { start: Position::new(9), end: Position::new(10) };
    let repetition = Repetition { span, op: RepetitionOp::Star, greedy: true, ast: Box::new(Ast::Empty(span.clone())) };
    let ast = Ast::Repetition(repetition);
    let result = ast.span();
}

#[test]
fn test_span_group() {
    let span = Span { start: Position::new(11), end: Position::new(12) };
    let group = Group { span, kind: GroupKind::Capturing(0), ast: Box::new(Ast::Empty(span.clone())) };
    let ast = Ast::Group(group);
    let result = ast.span();
}

#[test]
fn test_span_alternation() {
    let span = Span { start: Position::new(13), end: Position::new(14) };
    let asts = vec![Ast::Empty(span.clone()), Ast::Literal(Literal { span: span.clone(), kind: LiteralKind::Unicode('b'), c: 'b' })];
    let alternation = Alternation { span, asts };
    let ast = Ast::Alternation(alternation);
    let result = ast.span();
}

#[test]
fn test_span_concat() {
    let span = Span { start: Position::new(15), end: Position::new(16) };
    let asts = vec![Ast::Empty(span.clone()), Ast::Dot(span.clone())];
    let concat = Concat { span, asts };
    let ast = Ast::Concat(concat);
    let result = ast.span();
}

