// Answer 0

#[test]
fn test_ast_span_empty() {
    let span = Span { start: Position { byte: 0 }, end: Position { byte: 1 } };
    let ast = Ast::Empty(span.clone());
    assert_eq!(ast.span(), &span);
}

#[test]
fn test_ast_span_flags() {
    let span = Span { start: Position { byte: 1 }, end: Position { byte: 5 } };
    let flags = SetFlags { span: span.clone(), flags: Flags::default() };
    let ast = Ast::Flags(flags);
    assert_eq!(ast.span(), &span);
}

#[test]
fn test_ast_span_literal() {
    let span = Span { start: Position { byte: 2 }, end: Position { byte: 3 } };
    let literal = Literal { span: span.clone(), kind: LiteralKind::Unicode('a'), c: 'a' };
    let ast = Ast::Literal(literal);
    assert_eq!(ast.span(), &span);
}

#[test]
fn test_ast_span_dot() {
    let span = Span { start: Position { byte: 4 }, end: Position { byte: 5 } };
    let ast = Ast::Dot(span.clone());
    assert_eq!(ast.span(), &span);
}

#[test]
fn test_ast_span_assertion() {
    let span = Span { start: Position { byte: 6 }, end: Position { byte: 7 } };
    let assertion = Assertion { span: span.clone(), kind: AssertionKind::StartOfLine };
    let ast = Ast::Assertion(assertion);
    assert_eq!(ast.span(), &span);
}

#[test]
fn test_ast_span_class() {
    let span = Span { start: Position { byte: 8 }, end: Position { byte: 9 } };
    let class = Class::Unicode(ClassUnicode { span: span.clone() });
    let ast = Ast::Class(class);
    assert_eq!(ast.span(), &span);
}

#[test]
fn test_ast_span_repetition() {
    let span = Span { start: Position { byte: 10 }, end: Position { byte: 11 } };
    let repetition = Repetition { span: span.clone(), op: RepetitionOp::Plus, greedy: true, ast: Box::new(Ast::Empty(span.clone())) };
    let ast = Ast::Repetition(repetition);
    assert_eq!(ast.span(), &span);
}

#[test]
fn test_ast_span_group() {
    let span = Span { start: Position { byte: 12 }, end: Position { byte: 13 } };
    let group = Group { span: span.clone(), kind: GroupKind::Capturing(1), ast: Box::new(Ast::Empty(span.clone())) };
    let ast = Ast::Group(group);
    assert_eq!(ast.span(), &span);
}

#[test]
fn test_ast_span_alternation() {
    let span = Span { start: Position { byte: 14 }, end: Position { byte: 15 } };
    let alternation = Alternation { span: span.clone(), asts: vec![Ast::Empty(span.clone()), Ast::Literal(Literal { span: span.clone(), kind: LiteralKind::Unicode('b'), c: 'b' })] };
    let ast = Ast::Alternation(alternation);
    assert_eq!(ast.span(), &span);
}

#[test]
fn test_ast_span_concat() {
    let span = Span { start: Position { byte: 16 }, end: Position { byte: 17 } };
    let concat = Concat { span: span.clone(), asts: vec![Ast::Empty(span.clone()), Ast::Literal(Literal { span: span.clone(), kind: LiteralKind::Unicode('c'), c: 'c' })] };
    let ast = Ast::Concat(concat);
    assert_eq!(ast.span(), &span);
}

