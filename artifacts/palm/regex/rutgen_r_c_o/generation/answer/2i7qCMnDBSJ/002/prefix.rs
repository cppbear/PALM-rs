// Answer 0

#[test]
fn test_has_subexprs_with_empty_alternation() {
    let span = Span { start: Position(0), end: Position(0) };
    let alternation = Alternation { span, asts: vec![] };
    let ast = Ast::Alternation(alternation);
    ast.has_subexprs();
}

#[test]
fn test_has_subexprs_with_single_alternation() {
    let span = Span { start: Position(0), end: Position(1) };
    let literal = Literal { span, kind: LiteralKind::Unicode('a'), c: 'a' };
    let alternation = Alternation { span, asts: vec![Ast::Literal(literal)] };
    let ast = Ast::Alternation(alternation);
    ast.has_subexprs();
}

#[test]
fn test_has_subexprs_with_multiple_alternations() {
    let span = Span { start: Position(0), end: Position(5) };
    let literal_a = Literal { span, kind: LiteralKind::Unicode('a'), c: 'a' };
    let literal_b = Literal { span, kind: LiteralKind::Unicode('b'), c: 'b' };
    let alternation = Alternation { span, asts: vec![Ast::Literal(literal_a), Ast::Literal(literal_b)] };
    let ast = Ast::Alternation(alternation);
    ast.has_subexprs();
}

#[test]
fn test_has_subexprs_with_complex_alternation() {
    let span = Span { start: Position(0), end: Position(8) };
    let repetition = Repetition { span, kind: RepetitionKind::ZeroOrMore, greedy: true, hir: Box::new(Ast::Dot(span)) };
    let alternation = Alternation { span, asts: vec![Ast::Repetition(repetition)] };
    let ast = Ast::Alternation(alternation);
    ast.has_subexprs();
}

#[test]
fn test_has_subexprs_with_nested_alternation() {
    let span = Span { start: Position(0), end: Position(10) };
    let inner_literal = Literal { span: Span { start: Position(1), end: Position(2) }, kind: LiteralKind::Unicode('x'), c: 'x' };
    let inner_alternation = Alternation { span, asts: vec![Ast::Literal(inner_literal)] };
    let outer_alternation = Alternation { span, asts: vec![Ast::Alternation(inner_alternation)] };
    let ast = Ast::Alternation(outer_alternation);
    ast.has_subexprs();
}

