// Answer 0

#[test]
fn test_span_empty() {
    let span = Span { start: Position(0), end: Position(1) };
    let ast = Ast::Empty(span);
    ast.span();
}

#[test]
fn test_span_flags() {
    let span = Span { start: Position(0), end: Position(1) };
    let flags = SetFlags { span, flags: Flags::new() };
    let ast = Ast::Flags(flags);
    ast.span();
}

#[test]
fn test_span_literal() {
    let span = Span { start: Position(0), end: Position(1) };
    let literal = Literal { span, kind: LiteralKind::Byte(b'a'), c: 'a' };
    let ast = Ast::Literal(literal);
    ast.span();
}

#[test]
fn test_span_dot() {
    let span = Span { start: Position(0), end: Position(1) };
    let ast = Ast::Dot(span);
    ast.span();
}

#[test]
fn test_span_assertion() {
    let span = Span { start: Position(0), end: Position(1) };
    let assertion = Assertion { span, kind: AssertionKind::WordBoundary };
    let ast = Ast::Assertion(assertion);
    ast.span();
}

#[test]
fn test_span_class() {
    let span = Span { start: Position(0), end: Position(1) };
    let class = Class::Unicode(ClassUnicode::new());
    let ast = Ast::Class(class);
    ast.span();
}

#[test]
fn test_span_repetition() {
    let span = Span { start: Position(0), end: Position(1) };
    let repetition = Repetition { span, op: RepetitionOp::Star, greedy: true, ast: Box::new(Ast::Literal(Literal::new('a'))) };
    let ast = Ast::Repetition(repetition);
    ast.span();
}

#[test]
fn test_span_group() {
    let span = Span { start: Position(0), end: Position(1) };
    let group = Group { span, kind: GroupKind::Capturing(0), hir: Box::new(Ast::Empty(span)) };
    let ast = Ast::Group(group);
    ast.span();
}

#[test]
fn test_span_alternation() {
    let span = Span { start: Position(0), end: Position(1) };
    let alternation = Alternation { span, asts: vec![Ast::Literal(Literal::new('a')), Ast::Literal(Literal::new('b'))] };
    let ast = Ast::Alternation(alternation);
    ast.span();
}

#[test]
fn test_span_concat() {
    let span = Span { start: Position(0), end: Position(1) };
    let concat = Concat { span, asts: vec![Ast::Literal(Literal::new('a')), Ast::Literal(Literal::new('b'))] };
    let ast = Ast::Concat(concat);
    ast.span();
}

