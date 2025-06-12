// Answer 0

#[test]
fn test_span_literal_unicode() {
    let span = Span { start: Position { byte: 0 }, end: Position { byte: 1 } };
    let literal = Literal { span, kind: LiteralKind::Unicode, c: '\u{0061}' }; // 'a'
    let ast = Ast::Literal(literal);
    let _ = ast.span();
}

#[test]
fn test_span_literal_byte() {
    let span = Span { start: Position { byte: 0 }, end: Position { byte: 1 } };
    let literal = Literal { span, kind: LiteralKind::Byte, c: 'b' as u8 }; // 'b'
    let ast = Ast::Literal(literal);
    let _ = ast.span();
}

#[test]
fn test_span_empty() {
    let span = Span { start: Position { byte: 0 }, end: Position { byte: 1 } };
    let ast = Ast::Empty(span);
    let _ = ast.span();
}

#[test]
fn test_span_flags() {
    let span = Span { start: Position { byte: 0 }, end: Position { byte: 4 } };
    let flags = SetFlags { span, flags: Flags::default() };
    let ast = Ast::Flags(flags);
    let _ = ast.span();
}

#[test]
fn test_span_dot() {
    let span = Span { start: Position { byte: 0 }, end: Position { byte: 1 } };
    let ast = Ast::Dot(span);
    let _ = ast.span();
}

#[test]
fn test_span_assertion() {
    let span = Span { start: Position { byte: 0 }, end: Position { byte: 1 } };
    let assertion = Assertion { span, kind: AssertionKind::Start };
    let ast = Ast::Assertion(assertion);
    let _ = ast.span();
}

#[test]
fn test_span_class_unicode() {
    let span = Span { start: Position { byte: 0 }, end: Position { byte: 2 } };
    let class = Class::Unicode(ClassUnicode { span });
    let ast = Ast::Class(class);
    let _ = ast.span();
}

#[test]
fn test_span_repetition() {
    let span = Span { start: Position { byte: 0 }, end: Position { byte: 3 } };
    let repetition = Repetition { span, op: RepetitionOp::Star, greedy: true, ast: Box::new(Ast::Literal(Literal { span, kind: LiteralKind::Unicode, c: 'c' })) };
    let ast = Ast::Repetition(repetition);
    let _ = ast.span();
}

#[test]
fn test_span_group() {
    let span = Span { start: Position { byte: 0 }, end: Position { byte: 2 } };
    let group = Group { span, kind: GroupKind::Capturing(0), ast: Box::new(Ast::Literal(Literal { span, kind: LiteralKind::Unicode, c: 'd' })) };
    let ast = Ast::Group(group);
    let _ = ast.span();
}

#[test]
fn test_span_alternation() {
    let span = Span { start: Position { byte: 0 }, end: Position { byte: 4 } };
    let alternation = Alternation { span, asts: vec![Ast::Literal(Literal { span, kind: LiteralKind::Unicode, c: 'e' })] };
    let ast = Ast::Alternation(alternation);
    let _ = ast.span();
}

#[test]
fn test_span_concat() {
    let span = Span { start: Position { byte: 0 }, end: Position { byte: 3 } };
    let concat = Concat { span, asts: vec![Ast::Literal(Literal { span, kind: LiteralKind::Unicode, c: 'f' }), Ast::Literal(Literal { span, kind: LiteralKind::Unicode, c: 'g' })] };
    let ast = Ast::Concat(concat);
    let _ = ast.span();
}

