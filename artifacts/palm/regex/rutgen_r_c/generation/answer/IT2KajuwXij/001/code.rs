// Answer 0

#[test]
fn test_is_anchored_end_with_anchored_expression() {
    let hir = Hir {
        kind: HirKind::Literal(Literal::from_char('a')),
        info: HirInfo { bools: 0b00000001 },
    };
    assert!(hir.is_anchored_end());
}

#[test]
fn test_is_anchored_end_with_alternation_of_anchored_expressions() {
    let expr_a = Hir {
        kind: HirKind::Literal(Literal::from_char('a')),
        info: HirInfo { bools: 0b00000001 },
    };
    let expr_b = Hir {
        kind: HirKind::Literal(Literal::from_char('b')),
        info: HirInfo { bools: 0b00000001 },
    };
    let hir = Hir {
        kind: HirKind::Alternation(vec![expr_a, expr_b]),
        info: HirInfo { bools: 0b00000001 },
    };
    assert!(hir.is_anchored_end());
}

#[test]
fn test_is_anchored_end_with_non_anchored_expression() {
    let hir = Hir {
        kind: HirKind::Literal(Literal::from_char('a')),
        info: HirInfo { bools: 0b00000000 },
    };
    assert!(!hir.is_anchored_end());
}

#[test]
fn test_is_anchored_end_with_concat_of_non_anchored_and_anchored_expression() {
    let expr_non_anchored = Hir {
        kind: HirKind::Literal(Literal::from_char('a')),
        info: HirInfo { bools: 0b00000000 },
    };
    let expr_anchored = Hir {
        kind: HirKind::Literal(Literal::from_char('b')),
        info: HirInfo { bools: 0b00000001 },
    };
    let hir = Hir {
        kind: HirKind::Concat(vec![expr_non_anchored, expr_anchored]),
        info: HirInfo { bools: 0b00000001 },
    };
    assert!(!hir.is_anchored_end());
}

#[test]
fn test_is_anchored_end_with_grouping_of_anchored_expression() {
    let expr = Hir {
        kind: HirKind::Literal(Literal::from_char('c')),
        info: HirInfo { bools: 0b00000001 },
    };
    let hir = Hir {
        kind: HirKind::Group(Group::new(vec![expr])),
        info: HirInfo { bools: 0b00000001 },
    };
    assert!(hir.is_anchored_end());
}

