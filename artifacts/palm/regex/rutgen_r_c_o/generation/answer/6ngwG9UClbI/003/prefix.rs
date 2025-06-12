// Answer 0

#[test]
fn test_span_group_empty_span() {
    let span = Span { start: Position(0), end: Position(0)};
    let group = Group { span: span.clone(), kind: GroupKind::Capturing(0), ast: Box::new(Ast::Empty(span.clone())) };
    let _ = group.span();
}

#[test]
fn test_span_group_single_literal() {
    let span = Span { start: Position(0), end: Position(1)};
    let literal = Literal { span: span.clone(), kind: LiteralKind::Unicode('a'), c: 'a' };
    let group = Group { span: span.clone(), kind: GroupKind::Capturing(1), ast: Box::new(Ast::Literal(literal)) };
    let _ = group.span();
}

#[test]
fn test_span_group_dot() {
    let span = Span { start: Position(1), end: Position(1)};
    let group = Group { span: span.clone(), kind: GroupKind::Capturing(2), ast: Box::new(Ast::Dot(span.clone())) };
    let _ = group.span();
}

#[test]
fn test_span_group_repetition() {
    let span = Span { start: Position(1), end: Position(2)};
    let repetition = Repetition { span: span.clone(), op: RepetitionOp::ZeroOrMore, greedy: true, ast: Box::new(Ast::Literal(Literal { span: span.clone(), kind: LiteralKind::Unicode('b'), c: 'b' })) };
    let group = Group { span: span.clone(), kind: GroupKind::Capturing(3), ast: Box::new(Ast::Repetition(repetition)) };
    let _ = group.span();
}

#[test]
fn test_span_group_assertion() {
    let span = Span { start: Position(2), end: Position(2)};
    let assertion = Assertion { span: span.clone(), kind: AssertionKind::Boru };
    let group = Group { span: span.clone(), kind: GroupKind::Capturing(4), ast: Box::new(Ast::Assertion(assertion)) };
    let _ = group.span();
}

#[test]
fn test_span_group_alternation() {
    let span = Span { start: Position(5), end: Position(10)};
    let alternation = Alternation { span: span.clone(), asts: vec![Ast::Empty(span.clone()), Ast::Literal(Literal { span: span.clone(), kind: LiteralKind::Unicode('c'), c: 'c' })] };
    let group = Group { span: span.clone(), kind: GroupKind::Capturing(5), ast: Box::new(Ast::Alternation(alternation)) };
    let _ = group.span();
}

#[test]
fn test_span_group_concat() {
    let span = Span { start: Position(10), end: Position(20)};
    let concat = Concat { span: span.clone(), asts: vec![Ast::Literal(Literal { span: span.clone(), kind: LiteralKind::Unicode('d'), c: 'd' })] };
    let group = Group { span: span.clone(), kind: GroupKind::Capturing(6), ast: Box::new(Ast::Concat(concat)) };
    let _ = group.span();
}

