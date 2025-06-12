// Answer 0

#[test]
fn test_span_empty() {
    let span = Span { start: Position(0), end: Position(1) };
    let ast = Ast::Empty(span.clone());
    assert_eq!(ast.span(), &span);
}

#[test]
fn test_span_flags() {
    let span = Span { start: Position(2), end: Position(3) };
    let flags = SetFlags { span: span.clone(), flags: Flags::default() };
    let ast = Ast::Flags(flags);
    assert_eq!(ast.span(), &span);
}

#[test]
fn test_span_literal() {
    let span = Span { start: Position(4), end: Position(5) };
    let literal = Literal { span: span.clone(), kind: LiteralKind::Unicode('a'), c: 'a' };
    let ast = Ast::Literal(literal);
    assert_eq!(ast.span(), &span);
}

#[test]
fn test_span_dot() {
    let span = Span { start: Position(6), end: Position(7) };
    let ast = Ast::Dot(span.clone());
    assert_eq!(ast.span(), &span);
}

#[test]
fn test_span_assertion() {
    let span = Span { start: Position(8), end: Position(9) };
    let assertion = Assertion { span: span.clone(), kind: AssertionKind::Begin };
    let ast = Ast::Assertion(assertion);
    assert_eq!(ast.span(), &span);
}

#[test]
fn test_span_class_perl() {
    let span = Span { start: Position(10), end: Position(11) };
    let class = Class::Perl(ClassPerl { span: span.clone() });
    let ast = Ast::Class(class);
    assert_eq!(ast.span(), &span);
}

#[test]
fn test_span_repetition() {
    let span = Span { start: Position(12), end: Position(13) };
    let repetition = Repetition { span: span.clone(), op: RepetitionOp::Plus, greedy: true, ast: Box::new(Ast::Empty(span.clone())) };
    let ast = Ast::Repetition(repetition);
    assert_eq!(ast.span(), &span);
}

#[test]
fn test_span_group() {
    let span = Span { start: Position(14), end: Position(15) };
    let group = Group { span: span.clone(), kind: GroupKind::Capturing(1), ast: Box::new(Ast::Empty(span.clone())) };
    let ast = Ast::Group(group);
    assert_eq!(ast.span(), &span);
}

#[test]
fn test_span_alternation() {
    let span = Span { start: Position(16), end: Position(17) };
    let alternation = Alternation { span: span.clone(), asts: vec![Ast::Empty(span.clone())] };
    let ast = Ast::Alternation(alternation);
    assert_eq!(ast.span(), &span);
}

#[test]
fn test_span_concat() {
    let span = Span { start: Position(18), end: Position(19) };
    let concat = Concat { span: span.clone(), asts: vec![Ast::Empty(span.clone()), Ast::Empty(span.clone())] };
    let ast = Ast::Concat(concat);
    assert_eq!(ast.span(), &span);
}

