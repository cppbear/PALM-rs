// Answer 0

#[test]
fn test_is_empty_non_empty_ast() {
    let span = Span { start: Position(0), end: Position(1) };
    let literal = Literal::Unicode('a');
    let ast = Ast::Literal(literal);
    
    assert_eq!(ast.is_empty(), false);
}

#[test]
fn test_is_empty_non_empty_concat() {
    let span = Span { start: Position(0), end: Position(2) };
    let asts = vec![Ast::Literal(Literal::Unicode('a')), Ast::Literal(Literal::Unicode('b'))];
    let concat_ast = Ast::Concat(Concat { span, asts });

    assert_eq!(concat_ast.is_empty(), false);
}

#[test]
fn test_is_empty_non_empty_alternation() {
    let span = Span { start: Position(0), end: Position(3) };
    let asts = vec![Ast::Literal(Literal::Unicode('a')), Ast::Literal(Literal::Unicode('b'))];
    let alternation_ast = Ast::Alternation(Alternation { span, asts });

    assert_eq!(alternation_ast.is_empty(), false);
}

#[test]
fn test_is_empty_non_empty_repetition() {
    let span = Span { start: Position(0), end: Position(1) };
    let repetition_ast = Ast::Repetition(Repetition { span, kind: RepetitionKind::ZeroOrMore, greedy: true, hir: Box::new(Ast::Literal(Literal::Unicode('c'))) });

    assert_eq!(repetition_ast.is_empty(), false);
}

#[test]
fn test_is_empty_non_empty_group() {
    let span = Span { start: Position(0), end: Position(2) };
    let group_ast = Ast::Group(Group { span, kind: GroupKind::Capturing(1), hir: Box::new(Ast::Literal(Literal::Unicode('d'))) });

    assert_eq!(group_ast.is_empty(), false);
}

