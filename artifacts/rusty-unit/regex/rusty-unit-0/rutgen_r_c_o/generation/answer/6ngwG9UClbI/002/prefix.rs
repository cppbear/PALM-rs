// Answer 0

#[test]
fn test_ast_empty_span() {
    let span = Span { start: Position(0), end: Position(1) };
    let ast = Ast::Empty(span.clone());
    let result = ast.span();
}

#[test]
fn test_ast_flags_span() {
    let span = Span { start: Position(0), end: Position(3) };
    let flags = Flags { span: span.clone() };
    let ast = Ast::Flags(flags);
    let result = ast.span();
}

#[test]
fn test_ast_literal_span() {
    let span = Span { start: Position(2), end: Position(3) };
    let literal = Literal { span: span.clone(), kind: LiteralKind::Unicode, c: 'a' };
    let ast = Ast::Literal(literal);
    let result = ast.span();
}

#[test]
fn test_ast_dot_span() {
    let span = Span { start: Position(3), end: Position(4) };
    let ast = Ast::Dot(span.clone());
    let result = ast.span();
}

#[test]
fn test_ast_assertion_span() {
    let span = Span { start: Position(0), end: Position(2) };
    let assertion = Assertion { span: span.clone(), kind: AssertionKind::Start };
    let ast = Ast::Assertion(assertion);
    let result = ast.span();
}

#[test]
fn test_ast_class_span() {
    let span = Span { start: Position(5), end: Position(6) };
    let class = Class::Unicode(ClassUnicode { span: span.clone() });
    let ast = Ast::Class(class);
    let result = ast.span();
}

#[test]
fn test_ast_repetition_span() {
    let span = Span { start: Position(1), end: Position(4) };
    let repetition = Repetition { span: span.clone(), op: RepetitionOp::Star, greedy: true, ast: Box::new(Ast::Empty(span.clone())) };
    let ast = Ast::Repetition(repetition);
    let result = ast.span();
}

#[test]
fn test_ast_group_span() {
    let span = Span { start: Position(0), end: Position(5) };
    let group = Group { span: span.clone(), kind: GroupKind::Capturing(1), ast: Box::new(Ast::Empty(span.clone())) };
    let ast = Ast::Group(group);
    let result = ast.span();
}

#[test]
fn test_ast_alternation_span() {
    let span = Span { start: Position(0), end: Position(10) };
    let alternation = Alternation { span: span.clone(), asts: vec![Ast::Empty(span.clone()), Ast::Literal(Literal {span: span.clone(), kind: LiteralKind::Unicode, c: 'b'})]};
    let ast = Ast::Alternation(alternation);
    let result = ast.span();
}

#[test]
fn test_ast_concat_span() {
    let span = Span { start: Position(2), end: Position(8) };
    let concat = Concat { span: span.clone(), asts: vec![Ast::Empty(span.clone()), Ast::Group(Group { span: span.clone(), kind: GroupKind::Capturing(0), ast: Box::new(Ast::Literal(Literal {span: span.clone(), kind: LiteralKind::Unicode, c: 'c'}))})]};
    let ast = Ast::Concat(concat);
    let result = ast.span();
}

