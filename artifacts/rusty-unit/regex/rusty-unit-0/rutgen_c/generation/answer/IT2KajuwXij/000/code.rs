// Answer 0

#[test]
fn test_is_anchored_end_literal() {
    let hir = Hir {
        kind: HirKind::Literal(Literal::new('a')),
        info: HirInfo { bools: 0b00000000 },
    };
    assert!(!hir.is_anchored_end());
}

#[test]
fn test_is_anchored_end_concat() {
    let hir = Hir {
        kind: HirKind::Concat(vec![
            Hir::literal(Literal::new('f')),
            Hir::literal(Literal::new('o')),
            Hir::literal(Literal::new('o')),
            Hir::anchor(Anchor::new_end()),
        ]),
        info: HirInfo { bools: 0b00000001 }, // Assuming 0b00000001 indicates an anchored end
    };
    assert!(hir.is_anchored_end());
}

#[test]
fn test_is_anchored_end_alternation() {
    let hir = Hir {
        kind: HirKind::Alternation(vec![
            Hir::literal(Literal::new('f')),
            Hir::anchor(Anchor::new_end()),
        ]),
        info: HirInfo { bools: 0b00000001 }, // Assuming 0b00000001 indicates an anchored end
    };
    assert!(hir.is_anchored_end());
}

#[test]
fn test_is_anchored_end_non_anchored() {
    let hir = Hir {
        kind: HirKind::Alternation(vec![
            Hir::literal(Literal::new('f')),
            Hir::literal(Literal::new('b')),
        ]),
        info: HirInfo { bools: 0b00000000 },
    };
    assert!(!hir.is_anchored_end());
}

