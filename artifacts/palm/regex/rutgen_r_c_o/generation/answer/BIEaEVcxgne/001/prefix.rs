// Answer 0

#[test]
fn test_hir_kind_literal() {
    let hir_kind = HirKind::Literal(Literal {
        span: Span::default(),
        kind: LiteralKind::Unicode('a'),
        c: 'a',
    });
    let result = hir_kind.is_empty();
}

#[test]
fn test_hir_kind_class() {
    let hir_kind = HirKind::Class(Class::Unicode(ClassUnicode::default()));
    let result = hir_kind.is_empty();
}

#[test]
fn test_hir_kind_anchor() {
    let hir_kind = HirKind::Anchor(Anchor::StartLine);
    let result = hir_kind.is_empty();
}

#[test]
fn test_hir_kind_word_boundary() {
    let hir_kind = HirKind::WordBoundary(WordBoundary::Unicode);
    let result = hir_kind.is_empty();
}

#[test]
fn test_hir_kind_repetition() {
    let hir_kind = HirKind::Repetition(Repetition {
        span: Span::default(),
        op: RepetitionOp::Plus,
        greedy: true,
        ast: Box::new(Ast::default()),
    });
    let result = hir_kind.is_empty();
}

#[test]
fn test_hir_kind_group() {
    let hir_kind = HirKind::Group(Group {
        span: Span::default(),
        kind: GroupKind::Capturing(0),
        hir: Box::new(Hir {
            kind: HirKind::Literal(Literal::Unicode('a')),
            info: HirInfo::default(),
        }),
    });
    let result = hir_kind.is_empty();
}

#[test]
fn test_hir_kind_concat() {
    let hir_kind = HirKind::Concat(vec![
        Hir {
            kind: HirKind::Literal(Literal::Unicode('b')),
            info: HirInfo::default(),
        },
        Hir {
            kind: HirKind::Literal(Literal::Unicode('c')),
            info: HirInfo::default(),
        },
    ]);
    let result = hir_kind.is_empty();
}

#[test]
fn test_hir_kind_alternation() {
    let hir_kind = HirKind::Alternation(vec![
        Hir {
            kind: HirKind::Literal(Literal::Unicode('d')),
            info: HirInfo::default(),
        },
        Hir {
            kind: HirKind::Literal(Literal::Unicode('e')),
            info: HirInfo::default(),
        },
    ]);
    let result = hir_kind.is_empty();
}

