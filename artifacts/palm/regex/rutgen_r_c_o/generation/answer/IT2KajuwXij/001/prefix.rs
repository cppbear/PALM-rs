// Answer 0

#[test]
fn test_is_anchored_end_simple() {
    let info = HirInfo { bools: 0b00000000 };
    let hir = Hir { kind: HirKind::Literal(Literal::new('a')), info };
    hir.is_anchored_end();
}

#[test]
fn test_is_anchored_end_with_anchor() {
    let info = HirInfo { bools: 0b00000001 }; // Assuming this represents an anchored end
    let hir = Hir { kind: HirKind::Anchor(Anchor::new()), info };
    hir.is_anchored_end();
}

#[test]
fn test_is_anchored_end_with_concatenation() {
    let info = HirInfo { bools: 0b00000001 }; // Assuming this represents an expression that requires an end anchor
    let concat_hir = Hir::concat(vec![
        Hir { kind: HirKind::Literal(Literal::new('f')), info: HirInfo { bools: 0b00000000 } },
        Hir { kind: HirKind::Literal(Literal::new('o')), info: HirInfo { bools: 0b00000000 } },
        Hir { kind: HirKind::Anchor(Anchor::new()), info },
    ]);
    concat_hir.is_anchored_end();
}

#[test]
fn test_is_anchored_end_with_alternation() {
    let info_anchored = HirInfo { bools: 0b00000001 };
    let info_non_anchored = HirInfo { bools: 0b00000000 };
    let alt_hir = Hir::alternation(vec![
        Hir { kind: HirKind::Literal(Literal::new('f')), info: info_non_anchored },
        Hir { kind: HirKind::Anchor(Anchor::new()), info: info_anchored },
    ]);
    alt_hir.is_anchored_end();
}

#[test]
fn test_is_anchored_end_unanchored() {
    let info = HirInfo { bools: 0b00000000 }; // No anchoring
    let hir = Hir { kind: HirKind::Concat(vec![
        Hir { kind: HirKind::Literal(Literal::new('f')), info: HirInfo { bools: 0b00000000 } },
        Hir { kind: HirKind::Literal(Literal::new('o')), info: HirInfo { bools: 0b00000000 } },
    ]), info };
    hir.is_anchored_end();
}

#[test]
fn test_is_anchored_end_multiple_expressions() {
    let info = HirInfo { bools: 0b00000001 }; // Assuming this indicates an anchored end
    let hir = Hir::alternation(vec![
        Hir { kind: HirKind::Literal(Literal::new('b')), info: HirInfo { bools: 0b00000000 }},
        Hir { kind: HirKind::Literal(Literal::new('a')), info },
    ]);
    hir.is_anchored_end();
}

#[test]
fn test_is_anchored_end_edge_case() {
    let info = HirInfo { bools: 0b00000001 }; // Edge case with minimum anchoring
    let single_hir = Hir::literal(Literal::new('c'));
    let hir = Hir::concat(vec![single_hir, Hir { kind: HirKind::Anchor(Anchor::new()), info }]);
    hir.is_anchored_end();
}

