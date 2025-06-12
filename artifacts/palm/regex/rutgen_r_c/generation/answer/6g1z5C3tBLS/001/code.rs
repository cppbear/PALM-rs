// Answer 0

#[test]
fn test_is_all_assertions_empty() {
    let hir = Hir {
        kind: HirKind::Empty,
        info: HirInfo { bools: 0 },
    };
    assert!(hir.is_all_assertions());
}

#[test]
fn test_is_all_assertions_literal() {
    let hir = Hir {
        kind: HirKind::Literal(Literal::new('a')),
        info: HirInfo { bools: 0 },
    };
    assert!(!hir.is_all_assertions());
}

#[test]
fn test_is_all_assertions_anchor() {
    let hir = Hir {
        kind: HirKind::Anchor(Anchor::new()),
        info: HirInfo { bools: 1 },
    };
    assert!(hir.is_all_assertions());
}

#[test]
fn test_is_all_assertions_word_boundary() {
    let hir = Hir {
        kind: HirKind::WordBoundary(WordBoundary::new()),
        info: HirInfo { bools: 1 },
    };
    assert!(hir.is_all_assertions());
}

#[test]
fn test_is_all_assertions_concat() {
    let hir = Hir {
        kind: HirKind::Concat(vec![
            Hir {
                kind: HirKind::Anchor(Anchor::new()),
                info: HirInfo { bools: 1 },
            },
            Hir {
                kind: HirKind::WordBoundary(WordBoundary::new()),
                info: HirInfo { bools: 1 },
            },
        ]),
        info: HirInfo { bools: 2 },
    };
    assert!(hir.is_all_assertions());
}

#[test]
fn test_is_all_assertions_alternation() {
    let hir = Hir {
        kind: HirKind::Alternation(vec![
            Hir {
                kind: HirKind::Anchor(Anchor::new()),
                info: HirInfo { bools: 1 },
            },
            Hir {
                kind: HirKind::WordBoundary(WordBoundary::new()),
                info: HirInfo { bools: 1 },
            },
        ]),
        info: HirInfo { bools: 2 },
    };
    assert!(hir.is_all_assertions());
}

#[test]
fn test_is_all_assertions_complex() {
    let hir = Hir {
        kind: HirKind::Group(Group::new(vec![
            Hir {
                kind: HirKind::Anchor(Anchor::new()),
                info: HirInfo { bools: 1 },
            },
            Hir {
                kind: HirKind::WordBoundary(WordBoundary::new()),
                info: HirInfo { bools: 1 },
            },
        ])),
        info: HirInfo { bools: 2 },
    };
    assert!(hir.is_all_assertions());
}

#[test]
fn test_is_all_assertions_not_all() {
    let hir = Hir {
        kind: HirKind::Repetition(Repetition::new(Hir {
            kind: HirKind::Literal(Literal::new('b')),
            info: HirInfo { bools: 0 },
        })),
        info: HirInfo { bools: 0 },
    };
    assert!(!hir.is_all_assertions());
}

