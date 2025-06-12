// Answer 0

#[test]
fn test_span_literal() {
    let span = Span { start: 0, end: 1 };
    let literal = Literal { span: span.clone(), kind: LiteralKind::Byte(b'a'), c: 'a' };
    let ast = Ast::Literal(literal);
    ast.span();
}

#[test]
fn test_span_empty() {
    let span = Span { start: 0, end: 0 };
    let ast = Ast::Empty(span);
    ast.span();
}

#[test]
fn test_span_dot() {
    let span = Span { start: 1, end: 2 };
    let ast = Ast::Dot(span);
    ast.span();
}

#[test]
fn test_span_assertion() {
    let span = Span { start: 2, end: 3 };
    let assertion = Assertion { span: span.clone(), kind: AssertionKind::Start };
    let ast = Ast::Assertion(assertion);
    ast.span();
}

#[test]
fn test_span_class_unicode() {
    let span = Span { start: 3, end: 4 };
    let class_unicode = Class::Unicode(ClassUnicode { span: span.clone() });
    let ast = Ast::Class(class_unicode);
    ast.span();
}

#[test]
fn test_span_class_perl() {
    let span = Span { start: 4, end: 5 };
    let class_perl = Class::Perl(ClassPerl { span: span.clone() });
    let ast = Ast::Class(class_perl);
    ast.span();
}

#[test]
fn test_span_repetition() {
    let span = Span { start: 5, end: 6 };
    let repetition = Repetition { span: span.clone(), op: RepetitionOp::Star, greedy: true, ast: Box::new(Ast::Empty(span.clone())) };
    let ast = Ast::Repetition(repetition);
    ast.span();
}

#[test]
fn test_span_group() {
    let span = Span { start: 6, end: 7 };
    let group = Group { span: span.clone(), kind: GroupKind::Capture(1), ast: Box::new(Ast::Empty(span.clone())) };
    let ast = Ast::Group(group);
    ast.span();
}

#[test]
fn test_span_alternation() {
    let span = Span { start: 7, end: 8 };
    let alternation = Alternation { span: span.clone(), asts: vec![Ast::Empty(span.clone()), Ast::Empty(span.clone())] };
    let ast = Ast::Alternation(alternation);
    ast.span();
}

#[test]
fn test_span_concat() {
    let span = Span { start: 8, end: 9 };
    let concat = Concat { span: span.clone(), asts: vec![Ast::Empty(span.clone()), Ast::Empty(span.clone())] };
    let ast = Ast::Concat(concat);
    ast.span();
}

