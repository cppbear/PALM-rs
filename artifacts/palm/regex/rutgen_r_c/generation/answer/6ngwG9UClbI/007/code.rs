// Answer 0

#[test]
fn test_ast_span_empty() {
    let span = Span { start: Position(0), end: Position(1) };
    let ast = Ast::Empty(span.clone());
    assert_eq!(ast.span(), &span);
}

#[test]
fn test_ast_span_flags() {
    let span = Span { start: Position(0), end: Position(1) };
    let set_flags = SetFlags { span: span.clone(), flags: Flags::default() };
    let ast = Ast::Flags(set_flags);
    assert_eq!(ast.span(), &span);
}

#[test]
fn test_ast_span_literal() {
    let span = Span { start: Position(0), end: Position(1) };
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
    let assertion = Assertion { span: span.clone(), kind: AssertionKind::StartOfLine };
    let ast = Ast::Assertion(assertion);
    assert_eq!(ast.span(), &span);
}

#[test]
fn test_ast_span_class() {
    let span = Span { start: Position(0), end: Position(1) };
    let class = Class::Perl(ClassPerl { span: span.clone(), kind: ClassPerlKind::Digit });
    let ast = Ast::Class(class);
    assert_eq!(ast.span(), &span);
}

#[test]
fn test_ast_span_repetition() {
    let span = Span { start: Position(0), end: Position(1) };
    let repetition = Repetition { span: span.clone(), op: RepetitionOp::Star, greedy: true, ast: Box::new(Ast::Empty(span.clone())) };
    let ast = Ast::Repetition(repetition);
    assert_eq!(ast.span(), &span);
}

#[test]
fn test_ast_span_group() {
    let span = Span { start: Position(0), end: Position(1) };
    let group = Group { span: span.clone(), kind: GroupKind::Capturing(0), ast: Box::new(Ast::Empty(span.clone())) };
    let ast = Ast::Group(group);
    assert_eq!(ast.span(), &span);
}

#[test]
fn test_ast_span_alternation() {
    let span = Span { start: Position(0), end: Position(1) };
    let alternation = Alternation { span: span.clone(), asts: vec![Ast::Empty(span.clone())] };
    let ast = Ast::Alternation(alternation);
    assert_eq!(ast.span(), &span);
}

#[test]
fn test_ast_span_concat() {
    let span = Span { start: Position(0), end: Position(1) };
    let concat = Concat { span: span.clone(), asts: vec![Ast::Empty(span.clone()), Ast::Empty(span.clone())] };
    let ast = Ast::Concat(concat);
    assert_eq!(ast.span(), &span);
}

