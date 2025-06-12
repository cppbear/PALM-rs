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
fn test_is_match_empty_optional() {
    let hir = Hir {
        kind: HirKind::Repetition(Repetition::new(Literal::from('a'), 0, 1)), // 0 or 1 times
        info: HirInfo { bools: 0b00000001 },
    };
    assert!(hir.is_match_empty());
}

#[test]
fn test_is_match_empty_zero_or_more() {
    let hir = Hir {
        kind: HirKind::Repetition(Repetition::new(Literal::from('a'), 0, u32::MAX)), // 0 or more times
        info: HirInfo { bools: 0 },
    };
    assert!(hir.is_match_empty());
}

#[test]
fn test_is_match_empty_one_or_more() {
    let hir = Hir {
        kind: HirKind::Repetition(Repetition::new(Literal::from('a'), 1, u32::MAX)), // 1 or more times
        info: HirInfo { bools: 0 },
    };
    assert!(!hir.is_match_empty());
}

#[test]
fn test_is_match_empty_group() {
    let hir = Hir {
        kind: HirKind::Group(Group::new(vec![])), // empty group
        info: HirInfo { bools: 0 },
    };
    assert!(hir.is_match_empty());
}

#[test]
fn test_is_match_empty_concatenation() {
    let hir = Hir {
        kind: HirKind::Concat(vec![
            Hir::literal(Literal::from('a')),
            Hir::literal(Literal::from('b')),
        ]),
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
fn test_is_match_empty_word_boundary() {
    let hir = Hir {
        kind: HirKind::WordBoundary(WordBoundary::new()),
        info: HirInfo { bools: 0 },
    };
    assert!(hir.is_match_empty());
}

#[test]
fn test_is_match_empty_anchor() {
    let hir = Hir {
        kind: HirKind::Anchor(Anchor::new()),
        info: HirInfo { bools: 0 },
    };
    assert!(hir.is_match_empty());
}

