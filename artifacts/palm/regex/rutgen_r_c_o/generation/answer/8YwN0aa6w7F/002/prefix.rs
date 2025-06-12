// Answer 0

#[test]
fn test_has_subexprs_concat_with_two_elements() {
    let subexpr1 = Hir { kind: HirKind::Literal(Literal { span: Span::default(), kind: LiteralKind::Unicode('a'), c: 'a' }), info: HirInfo::default() };
    let subexpr2 = Hir { kind: HirKind::Literal(Literal { span: Span::default(), kind: LiteralKind::Unicode('b'), c: 'b' }), info: HirInfo::default() };
    let concat_expr = HirKind::Concat(vec![subexpr1, subexpr2]);
    concat_expr.has_subexprs();
}

#[test]
fn test_has_subexprs_concat_with_three_elements() {
    let subexpr1 = Hir { kind: HirKind::Literal(Literal { span: Span::default(), kind: LiteralKind::Unicode('x'), c: 'x' }), info: HirInfo::default() };
    let subexpr2 = Hir { kind: HirKind::Literal(Literal { span: Span::default(), kind: LiteralKind::Unicode('y'), c: 'y' }), info: HirInfo::default() };
    let subexpr3 = Hir { kind: HirKind::Literal(Literal { span: Span::default(), kind: LiteralKind::Unicode('z'), c: 'z' }), info: HirInfo::default() };
    let concat_expr = HirKind::Concat(vec![subexpr1, subexpr2, subexpr3]);
    concat_expr.has_subexprs();
}

#[test]
fn test_has_subexprs_concat_with_nested_group() {
    let inner_group = Hir { kind: HirKind::Group(Group { span: Span::default(), kind: GroupKind::Capturing(0), hir: Box::new(Hir { kind: HirKind::Literal(Literal { span: Span::default(), kind: LiteralKind::Unicode('c'), c: 'c' }), info: HirInfo::default() }) }) , info: HirInfo::default() };
    let outer_group = Hir { kind: HirKind::Group(Group { span: Span::default(), kind: GroupKind::Capturing(1), hir: Box::new(inner_group) }), info: HirInfo::default() };
    let concat_expr = HirKind::Concat(vec![outer_group, Hir { kind: HirKind::Literal(Literal { span: Span::default(), kind: LiteralKind::Unicode('d'), c: 'd' }), info: HirInfo::default() }]);
    concat_expr.has_subexprs();
}

#[test]
fn test_has_subexprs_concat_with_repetition() {
    let repeat_expr = Hir { kind: HirKind::Repetition(Repetition { span: Span::default(), op: RepetitionOp::ZeroOrMore, greedy: true, hir: Box::new(Hir { kind: HirKind::Literal(Literal { span: Span::default(), kind: LiteralKind::Unicode('e'), c: 'e' }), info: HirInfo::default() }) }) , info: HirInfo::default() };
    let concat_expr = HirKind::Concat(vec![repeat_expr, Hir { kind: HirKind::Literal(Literal { span: Span::default(), kind: LiteralKind::Unicode('f'), c: 'f' }), info: HirInfo::default() }]);
    concat_expr.has_subexprs();
}

#[test]
fn test_has_subexprs_concat_with_alternation() {
    let alt_expr = HirKind::Alternation(vec![
        Hir { kind: HirKind::Literal(Literal { span: Span::default(), kind: LiteralKind::Unicode('g'), c: 'g' }), info: HirInfo::default() },
        Hir { kind: HirKind::Literal(Literal { span: Span::default(), kind: LiteralKind::Unicode('h'), c: 'h' }), info: HirInfo::default() }
    ]);
    let concat_expr = HirKind::Concat(vec![alt_expr, Hir { kind: HirKind::Literal(Literal { span: Span::default(), kind: LiteralKind::Unicode('i'), c: 'i' }), info: HirInfo::default() }]);
    concat_expr.has_subexprs();
}

