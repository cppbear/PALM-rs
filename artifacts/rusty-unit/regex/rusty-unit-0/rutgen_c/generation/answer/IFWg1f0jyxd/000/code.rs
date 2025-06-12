// Answer 0

#[test]
fn test_is_match_empty_empty() {
    let hir = Hir {
        kind: HirKind::Empty,
        info: HirInfo { bools: 0 },
    };
    assert!(hir.is_match_empty());
}

#[test]
fn test_is_match_empty_literal() {
    let hir = Hir {
        kind: HirKind::Literal(Literal::from('a')),
        info: HirInfo { bools: 0 },
    };
    assert!(!hir.is_match_empty());
}

#[test]
fn test_is_match_empty_alternation() {
    let hir = Hir {
        kind: HirKind::Alternation(vec![
            Hir::literal(Literal::from('a')),
            Hir::empty(),
        ]),
        info: HirInfo { bools: 0 },
    };
    assert!(hir.is_match_empty());
}

#[test]
fn test_is_match_empty_repetition() {
    let hir = Hir {
        kind: HirKind::Repetition(Repetition { /* suitable fields */ }),
        info: HirInfo { bools: 0 },
    };
    assert!(hir.is_match_empty());
}

#[test]
fn test_is_match_empty_anchored() {
    let hir = Hir {
        kind: HirKind::Anchor(Anchor::Start),
        info: HirInfo { bools: 0 },
    };
    assert!(!hir.is_match_empty());
}

#[test]
fn test_is_match_empty_group() {
    let hir = Hir {
        kind: HirKind::Group(Group {
            // suitable fields
        }),
        info: HirInfo { bools: 0 },
    };
    assert!(hir.is_match_empty());
}

