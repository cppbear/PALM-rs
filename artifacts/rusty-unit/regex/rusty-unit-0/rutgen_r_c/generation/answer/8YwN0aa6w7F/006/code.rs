// Answer 0

#[test]
fn test_hirkind_empty() {
    let kind = HirKind::Empty;
    assert_eq!(kind.has_subexprs(), false);
}

#[test]
fn test_hirkind_literal() {
    let kind = HirKind::Literal(Literal {
        span: Span::default(),
        kind: LiteralKind::Unicode('a'),
        c: 'a',
    });
    assert_eq!(kind.has_subexprs(), false);
}

#[test]
fn test_hirkind_anchor_start_line() {
    let kind = HirKind::Anchor(Anchor::StartLine);
    assert_eq!(kind.has_subexprs(), false);
}

#[test]
fn test_hirkind_word_boundary_unicode() {
    let kind = HirKind::WordBoundary(WordBoundary::Unicode);
    assert_eq!(kind.has_subexprs(), false);
}

#[test]
fn test_hirkind_class_unicode() {
    let kind = HirKind::Class(Class::Unicode(ClassUnicode::default()));
    assert_eq!(kind.has_subexprs(), false);
}

#[test]
fn test_hirkind_group() {
    let kind = HirKind::Group(Group {
        span: Span::default(),
        kind: GroupKind::Capture(0, None),
        hir: Box::new(Hir {
            kind: HirKind::Literal(Literal {
                span: Span::default(),
                kind: LiteralKind::Unicode('a'),
                c: 'a',
            }),
            info: HirInfo::default(),
        }),
    });
    assert_eq!(kind.has_subexprs(), true);
}

#[test]
fn test_hirkind_repetition() {
    let kind = HirKind::Repetition(Repetition {
        span: Span::default(),
        op: RepetitionOp::ZeroOrMore,
        greedy: true,
        ast: Box::new(Hir {
            kind: HirKind::Literal(Literal {
                span: Span::default(),
                kind: LiteralKind::Unicode('a'),
                c: 'a',
            }),
            info: HirInfo::default(),
        }),
    });
    assert_eq!(kind.has_subexprs(), true);
}

#[test]
fn test_hirkind_concat() {
    let kind = HirKind::Concat(vec![
        Hir {
            kind: HirKind::Literal(Literal {
                span: Span::default(),
                kind: LiteralKind::Unicode('a'),
                c: 'a',
            }),
            info: HirInfo::default(),
        },
        Hir {
            kind: HirKind::Literal(Literal {
                span: Span::default(),
                kind: LiteralKind::Unicode('b'),
                c: 'b',
            }),
            info: HirInfo::default(),
        },
    ]);
    assert_eq!(kind.has_subexprs(), true);
}

#[test]
fn test_hirkind_alternation() {
    let kind = HirKind::Alternation(vec![
        Hir {
            kind: HirKind::Literal(Literal {
                span: Span::default(),
                kind: LiteralKind::Unicode('a'),
                c: 'a',
            }),
            info: HirInfo::default(),
        },
        Hir {
            kind: HirKind::Literal(Literal {
                span: Span::default(),
                kind: LiteralKind::Unicode('b'),
                c: 'b',
            }),
            info: HirInfo::default(),
        },
    ]);
    assert_eq!(kind.has_subexprs(), true);
}

