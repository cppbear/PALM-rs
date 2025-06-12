// Answer 0

#[test]
fn test_has_subexprs_empty() {
    let span = Span { start: Position(0), end: Position(0) };
    let ast = Ast::Empty(span);
    assert_eq!(ast.has_subexprs(), false);
}

#[test]
fn test_has_subexprs_literal() {
    let span = Span { start: Position(0), end: Position(1) };
    let literal = Literal { span, kind: LiteralKind::Unicode('a'), c: 'a' };
    let ast = Ast::Literal(literal);
    assert_eq!(ast.has_subexprs(), false);
}

#[test]
fn test_has_subexprs_flags() {
    let span = Span { start: Position(0), end: Position(5) };
    let flags = SetFlags { span, flags: Flags::empty() };  // Assume Flags::empty() is available
    let ast = Ast::Flags(flags);
    assert_eq!(ast.has_subexprs(), false);
}

#[test]
fn test_has_subexprs_assertion() {
    let span = Span { start: Position(0), end: Position(1) };
    let assertion = Assertion { span, kind: AssertionKind::WordBoundary }; // Assume AssertionKind::WordBoundary is a valid variant
    let ast = Ast::Assertion(assertion);
    assert_eq!(ast.has_subexprs(), false);
}

#[test]
fn test_has_subexprs_dot() {
    let span = Span { start: Position(0), end: Position(1) };
    let ast = Ast::Dot(span);
    assert_eq!(ast.has_subexprs(), false);
}

#[test]
fn test_has_subexprs_repetition() {
    let span = Span { start: Position(0), end: Position(1) };
    let inner_ast = Ast::Literal(Literal { span, kind: LiteralKind::Unicode('a'), c: 'a' });
    let repetition = Repetition { span, kind: RepetitionKind::Star, greedy: true, hir: Box::new(inner_ast) };
    let ast = Ast::Repetition(repetition);
    assert_eq!(ast.has_subexprs(), true);
}

#[test]
fn test_has_subexprs_group() {
    let span = Span { start: Position(0), end: Position(1) };
    let inner_ast = Ast::Literal(Literal { span, kind: LiteralKind::Unicode('b'), c: 'b' });
    let group = Group { span, kind: GroupKind::Capturing(0), hir: Box::new(inner_ast) };
    let ast = Ast::Group(group);
    assert_eq!(ast.has_subexprs(), true);
}

#[test]
fn test_has_subexprs_alternation() {
    let span = Span { start: Position(0), end: Position(1) };
    let ast1 = Ast::Literal(Literal { span, kind: LiteralKind::Unicode('c'), c: 'c' });
    let ast2 = Ast::Literal(Literal { span, kind: LiteralKind::Unicode('d'), c: 'd' });
    let alternation = Alternation { span, asts: vec![ast1, ast2] };
    let ast = Ast::Alternation(alternation);
    assert_eq!(ast.has_subexprs(), true);
}

#[test]
fn test_has_subexprs_concat() {
    let span = Span { start: Position(0), end: Position(2) };
    let ast1 = Ast::Literal(Literal { span, kind: LiteralKind::Unicode('e'), c: 'e' });
    let ast2 = Ast::Literal(Literal { span, kind: LiteralKind::Unicode('f'), c: 'f' });
    let concat = Concat { span, asts: vec![ast1, ast2] };
    let ast = Ast::Concat(concat);
    assert_eq!(ast.has_subexprs(), true);
}

