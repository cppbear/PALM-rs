// Answer 0

#[test]
fn test_ast_span_empty() {
    let span = Span { start: Position(0), end: Position(0) };
    let ast = Ast::Empty(span.clone());
    assert_eq!(ast.span(), &span);
}

#[test]
fn test_ast_span_flags() {
    let span = Span { start: Position(0), end: Position(5) };
    let flags = SetFlags { span: span.clone(), flags: Flags::default() };
    let ast = Ast::Flags(flags);
    assert_eq!(ast.span(), &span);
}

#[test]
fn test_ast_span_literal() {
    let span = Span { start: Position(1), end: Position(2) };
    let literal = Literal { span: span.clone(), kind: LiteralKind::Unicode('a'), c: 'a' };
    let ast = Ast::Literal(literal);
    assert_eq!(ast.span(), &span);
}

#[test]
fn test_ast_span_dot() {
    let span = Span { start: Position(0), end: Position(1) };
    let ast = Ast::Dot(span.clone());
    assert_eq!(ast.span(), &span);
}

#[test]
fn test_ast_span_assertion() {
    let span = Span { start: Position(0), end: Position(1) };
    let assertion = Assertion { span: span.clone(), kind: AssertionKind::StartOfInput };
    let ast = Ast::Assertion(assertion);
    assert_eq!(ast.span(), &span);
}

#[test]
fn test_ast_span_class() {
    let span = Span { start: Position(0), end: Position(3) };
    let class = Class::Unicode(ClassUnicode::default());
    let ast = Ast::Class(class);
    assert_eq!(ast.span(), &span);
}

#[test]
fn test_ast_span_repetition() {
    let span = Span { start: Position(0), end: Position(3) };
    let repetition = Repetition { span: span.clone(), op: RepetitionOp::ZeroOrMore, greedy: true, ast: Box::new(Ast::Empty(span.clone())) };
    let ast = Ast::Repetition(repetition);
    assert_eq!(ast.span(), &span);
}

#[test]
fn test_ast_span_group() {
    let span = Span { start: Position(0), end: Position(3) };
    let group = Group { span: span.clone(), kind: GroupKind::Capturing(1), ast: Box::new(Ast::Empty(span.clone())) };
    let ast = Ast::Group(group);
    assert_eq!(ast.span(), &span);
}

#[test]
fn test_ast_span_alternation() {
    let span = Span { start: Position(0), end: Position(3) };
    let alt = Alternation { span: span.clone(), asts: vec![Ast::Empty(span.clone()), Ast::Literal(Literal { span: span.clone(), kind: LiteralKind::Unicode('b'), c: 'b' })] };
    let ast = Ast::Alternation(alt);
    assert_eq!(ast.span(), &span);
}

#[test]
fn test_ast_span_concat() {
    let span = Span { start: Position(0), end: Position(6) };
    let concat = Concat { span: span.clone(), asts: vec![Ast::Empty(span.clone()), Ast::Literal(Literal { span: span.clone(), kind: LiteralKind::Unicode('c'), c: 'c' })] };
    let ast = Ast::Concat(concat);
    assert_eq!(ast.span(), &span);
}

