// Answer 0

#[test]
fn test_ast_span_empty() {
    let span = Span { start: Position(0), end: Position(1) };
    let ast = Ast::Empty(span.clone());
    let _ = ast.span();
}

#[test]
fn test_ast_span_flags() {
    let span = Span { start: Position(2), end: Position(3) };
    let flags = SetFlags { span: span.clone(), flags: Flags::default() };
    let ast = Ast::Flags(flags);
    let _ = ast.span();
}

#[test]
fn test_ast_span_literal() {
    let span = Span { start: Position(4), end: Position(5) };
    let literal = Literal { span: span.clone(), kind: LiteralKind::Unicode, c: 'a' };
    let ast = Ast::Literal(literal);
    let _ = ast.span();
}

#[test]
fn test_ast_span_dot() {
    let span = Span { start: Position(6), end: Position(7) };
    let ast = Ast::Dot(span.clone());
    let _ = ast.span();
}

#[test]
fn test_ast_span_assertion() {
    let span = Span { start: Position(8), end: Position(9) };
    let assertion = Assertion { span: span.clone(), kind: AssertionKind::WordBoundary };
    let ast = Ast::Assertion(assertion);
    let _ = ast.span();
}

#[test]
fn test_ast_span_class_unicode() {
    let span = Span { start: Position(10), end: Position(11) };
    let class = Class::Unicode(ClassUnicode::default());
    let ast = Ast::Class(class);
    let _ = ast.span();
}

#[test]
fn test_ast_span_repetition() {
    let span = Span { start: Position(12), end: Position(13) };
    let repetition = Repetition { span: span.clone(), op: RepetitionOp::Star, greedy: true, ast: Box::new(Ast::Empty(span.clone())) };
    let ast = Ast::Repetition(repetition);
    let _ = ast.span();
}

#[test]
fn test_ast_span_group() {
    let span = Span { start: Position(14), end: Position(15) };
    let group = Group { span: span.clone(), kind: GroupKind::Capturing(1), ast: Box::new(Ast::Empty(span.clone())) };
    let ast = Ast::Group(group);
    let _ = ast.span();
}

#[test]
fn test_ast_span_alternation() {
    let span = Span { start: Position(16), end: Position(17) };
    let alternation = Alternation { span: span.clone(), asts: vec![Ast::Empty(span.clone())] };
    let ast = Ast::Alternation(alternation);
    let _ = ast.span();
}

#[test]
fn test_ast_span_concat() {
    let span = Span { start: Position(18), end: Position(19) };
    let concat = Concat { span: span.clone(), asts: vec![Ast::Empty(span.clone()), Ast::Literal(Literal { span: span.clone(), kind: LiteralKind::Unicode, c: 'b' })] };
    let ast = Ast::Concat(concat);
    let _ = ast.span();
}

