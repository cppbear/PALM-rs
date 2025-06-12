// Answer 0

#[test]
fn test_has_subexprs_with_alternation() {
    let literal_a = HirKind::Literal(Literal { span: Span::default(), kind: LiteralKind::Unicode('a'), c: 'a' });
    let literal_b = HirKind::Literal(Literal { span: Span::default(), kind: LiteralKind::Unicode('b'), c: 'b' });
    let alternation = HirKind::Alternation(vec![literal_a, literal_b]);
    alternation.has_subexprs();
}

#[test]
fn test_has_subexprs_with_empty_alternation() {
    let alternation = HirKind::Alternation(vec![]);
    alternation.has_subexprs();
}

#[test]
fn test_has_subexprs_with_multiple_literals_in_alternation() {
    let literal_a = HirKind::Literal(Literal { span: Span::default(), kind: LiteralKind::Unicode('a'), c: 'a' });
    let literal_b = HirKind::Literal(Literal { span: Span::default(), kind: LiteralKind::Unicode('b'), c: 'b' });
    let literal_c = HirKind::Literal(Literal { span: Span::default(), kind: LiteralKind::Unicode('c'), c: 'c' });
    let alternation = HirKind::Alternation(vec![literal_a, literal_b, literal_c]);
    alternation.has_subexprs();
}

#[test]
fn test_has_subexprs_with_nested_alternation() {
    let nested_alternation = HirKind::Alternation(vec![HirKind::Alternation(vec![
        HirKind::Literal(Literal { span: Span::default(), kind: LiteralKind::Unicode('d'), c: 'd' }),
        HirKind::Literal(Literal { span: Span::default(), kind: LiteralKind::Unicode('e'), c: 'e' })
    ])]);
    nested_alternation.has_subexprs();
}

