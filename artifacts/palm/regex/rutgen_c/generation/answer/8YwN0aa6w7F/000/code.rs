// Answer 0

#[test]
fn test_hirkind_empty_has_subexprs() {
    let kind = HirKind::Empty;
    assert!(!kind.has_subexprs());
}

#[test]
fn test_hirkind_literal_has_subexprs() {
    let kind = HirKind::Literal(Literal { span: Span::default(), kind: LiteralKind::Unicode('a'), c: 'a' });
    assert!(!kind.has_subexprs());
}

#[test]
fn test_hirkind_class_has_subexprs() {
    let kind = HirKind::Class(Class::Unicode(ClassUnicode::default()));
    assert!(!kind.has_subexprs());
}

#[test]
fn test_hirkind_anchor_has_subexprs() {
    let kind = HirKind::Anchor(Anchor::StartLine);
    assert!(!kind.has_subexprs());
}

#[test]
fn test_hirkind_word_boundary_has_subexprs() {
    let kind = HirKind::WordBoundary(WordBoundary::Unicode);
    assert!(!kind.has_subexprs());
}

#[test]
fn test_hirkind_group_has_subexprs() {
    let kind = HirKind::Group(Group { span: Span::default(), kind: GroupKind::Capturing(0), hir: Box::new(Hir { kind: HirKind::Empty, info: HirInfo::default() }) });
    assert!(kind.has_subexprs());
}

#[test]
fn test_hirkind_repetition_has_subexprs() {
    let kind = HirKind::Repetition(Repetition { span: Span::default(), op: RepetitionOp::Star, greedy: true, ast: Box::new(Ast::default()) });
    assert!(kind.has_subexprs());
}

#[test]
fn test_hirkind_concat_has_subexprs() {
    let kind = HirKind::Concat(vec![
        Hir { kind: HirKind::Empty, info: HirInfo::default() },
        Hir { kind: HirKind::Literal(Literal { span: Span::default(), kind: LiteralKind::Unicode('b'), c: 'b' }), info: HirInfo::default() },
    ]);
    assert!(kind.has_subexprs());
}

#[test]
fn test_hirkind_alternation_has_subexprs() {
    let kind = HirKind::Alternation(vec![
        Hir { kind: HirKind::Empty, info: HirInfo::default() },
        Hir { kind: HirKind::Literal(Literal { span: Span::default(), kind: LiteralKind::Unicode('c'), c: 'c' }), info: HirInfo::default() },
    ]);
    assert!(kind.has_subexprs());
}

