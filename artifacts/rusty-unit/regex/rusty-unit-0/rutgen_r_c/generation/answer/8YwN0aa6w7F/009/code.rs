// Answer 0

#[test]
fn test_hirkind_is_empty() {
    let kind = HirKind::Empty;
    assert_eq!(kind.has_subexprs(), false);
}

#[test]
fn test_hirkind_is_literal() {
    let kind = HirKind::Literal(Literal { span: Span::default(), kind: LiteralKind::Unicode('a'), c: 'a' });
    assert_eq!(kind.has_subexprs(), false);
}

#[test]
fn test_hirkind_is_class() {
    let kind = HirKind::Class(Class::Unicode(ClassUnicode::default()));
    assert_eq!(kind.has_subexprs(), false);
}

#[test]
fn test_hirkind_is_anchor() {
    let kind = HirKind::Anchor(Anchor::StartLine);
    assert_eq!(kind.has_subexprs(), false);
}

#[test]
fn test_hirkind_is_word_boundary() {
    let kind = HirKind::WordBoundary(WordBoundary::Unicode);
    assert_eq!(kind.has_subexprs(), false);
}

#[test]
fn test_hirkind_is_group() {
    let kind = HirKind::Group(Group { span: Span::default(), kind: GroupKind::default(), hir: Box::new(Hir { kind: HirKind::Empty, info: HirInfo::default() }) });
    assert_eq!(kind.has_subexprs(), true);
}

#[test]
fn test_hirkind_is_repetition() {
    let kind = HirKind::Repetition(Repetition { span: Span::default(), op: RepetitionOp::default(), greedy: true, ast: Box::new(Ast::default()) });
    assert_eq!(kind.has_subexprs(), true);
}

#[test]
fn test_hirkind_is_concat() {
    let kind = HirKind::Concat(vec![Hir { kind: HirKind::Empty, info: HirInfo::default() }]);
    assert_eq!(kind.has_subexprs(), true);
}

#[test]
fn test_hirkind_is_alternation() {
    let kind = HirKind::Alternation(vec![Hir { kind: HirKind::Empty, info: HirInfo::default() }, Hir { kind: HirKind::Literal(Literal { span: Span::default(), kind: LiteralKind::Unicode('b'), c: 'b' }) }]);
    assert_eq!(kind.has_subexprs(), true);
}

