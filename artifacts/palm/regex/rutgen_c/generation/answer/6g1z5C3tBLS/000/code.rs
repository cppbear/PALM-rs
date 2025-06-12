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
fn test_is_all_assertions_anchor() {
    let anchor = Anchor; // Assume Anchor is defined somewhere
    let hir = Hir {
        kind: HirKind::Anchor(anchor),
        info: HirInfo { bools: 0 },
    };
    assert!(hir.is_all_assertions());
}

#[test]
fn test_is_all_assertions_word_boundary() {
    let word_boundary = WordBoundary; // Assume WordBoundary is defined somewhere
    let hir = Hir {
        kind: HirKind::WordBoundary(word_boundary),
        info: HirInfo { bools: 0 },
    };
    assert!(hir.is_all_assertions());
}

#[test]
fn test_is_all_assertions_non_assertions() {
    let literal = Literal; // Assume Literal is defined somewhere
    let hir = Hir {
        kind: HirKind::Literal(literal),
        info: HirInfo { bools: 0 },
    };
    assert!(!hir.is_all_assertions());
}

#[test]
fn test_is_all_assertions_concatenation() {
    let anchor = Anchor; // Assume Anchor is defined somewhere
    let hir = Hir {
        kind: HirKind::Concat(vec![
            Hir {
                kind: HirKind::Anchor(anchor),
                info: HirInfo { bools: 0 },
            },
            Hir {
                kind: HirKind::Anchor(anchor),
                info: HirInfo { bools: 0 },
            },
        ]),
        info: HirInfo { bools: 0 },
    };
    assert!(hir.is_all_assertions());
}

