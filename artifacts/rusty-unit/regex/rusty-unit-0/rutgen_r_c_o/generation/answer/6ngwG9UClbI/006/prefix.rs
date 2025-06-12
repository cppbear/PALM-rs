// Answer 0

#[test]
fn test_span_empty() {
    let span = Span { start: Position(0), end: Position(1) };
    let ast = Ast::Empty(span.clone());
    ast.span(); // should return span
}

#[test]
fn test_span_flags() {
    let span = Span { start: Position(0), end: Position(1) };
    let flags = SetFlags { span: span.clone(), flags: Flags::new() };
    let ast = Ast::Flags(flags);
    ast.span(); // should return span
}

#[test]
fn test_span_literal() {
    let span = Span { start: Position(0), end: Position(1) };
    let literal = Literal { span: span.clone(), kind: LiteralKind::Unicode, c: 'a' };
    let ast = Ast::Literal(literal);
    ast.span(); // should return span
}

#[test]
fn test_span_dot() {
    let span = Span { start: Position(0), end: Position(1) };
    let ast = Ast::Dot(span.clone());
    ast.span(); // should return span
}

#[test]
fn test_span_assertion() {
    let span = Span { start: Position(0), end: Position(1) };
    let assertion = Assertion { span: span.clone(), kind: AssertionKind::StartOfText };
    let ast = Ast::Assertion(assertion);
    ast.span(); // should return span
}

#[test]
fn test_span_class() {
    let span = Span { start: Position(0), end: Position(1) };
    let class = Class::Unicode(ClassUnicode::new());
    let ast = Ast::Class(class);
    ast.span(); // should return span
}

#[test]
fn test_span_repetition() {
    let span = Span { start: Position(0), end: Position(1) };
    let repetition = Repetition { span: span.clone(), op: RepetitionOp::Star, greedy: true, ast: Box::new(Ast::Empty(span.clone())) };
    let ast = Ast::Repetition(repetition);
    ast.span(); // should return span
}

#[test]
fn test_span_group() {
    let span = Span { start: Position(0), end: Position(1) };
    let group = Group { span: span.clone(), kind: GroupKind::Capturing(1, None), ast: Box::new(Ast::Empty(span.clone())) };
    let ast = Ast::Group(group);
    ast.span(); // should return span
}

#[test]
fn test_span_alternation() {
    let span = Span { start: Position(0), end: Position(1) };
    let alternation = Alternation { span: span.clone(), asts: vec![Ast::Empty(span.clone())] };
    let ast = Ast::Alternation(alternation);
    ast.span(); // should return span
}

#[test]
fn test_span_concatenation() {
    let span = Span { start: Position(0), end: Position(1) };
    let concat = Concat { span: span.clone(), asts: vec![Ast::Empty(span.clone())] };
    let ast = Ast::Concat(concat);
    ast.span(); // should return span
}

