// Answer 0

#[test]
fn test_alternation_empty() {
    let exprs: Vec<Hir> = Vec::new();
    let result = Hir::alternation(exprs);
    assert_eq!(result.kind(), &HirKind::Empty);
}

#[test]
fn test_alternation_single() {
    let literal_hir = Hir::literal(Literal::from_char('a'));
    let exprs = vec![literal_hir.clone()];
    let result = Hir::alternation(exprs);
    assert_eq!(result.kind(), &HirKind::Literal(literal_hir.into_kind()));
}

#[test]
fn test_alternation_multiple() {
    let literal_hir_a = Hir::literal(Literal::from_char('a'));
    let literal_hir_b = Hir::literal(Literal::from_char('b'));
    let exprs = vec![literal_hir_a.clone(), literal_hir_b.clone()];
    let result = Hir::alternation(exprs);

    match result.kind() {
        HirKind::Alternation(kids) => {
            assert_eq!(kids.len(), 2);
            assert!(kids.contains(&literal_hir_a));
            assert!(kids.contains(&literal_hir_b));
        },
        _ => panic!("Expected type HirKind::Alternation"),
    }
    assert!(result.is_always_utf8());
    assert!(result.is_all_assertions());
    assert!(result.is_anchored_start());
    assert!(result.is_anchored_end());
    assert!(!result.is_any_anchored_start());
    assert!(!result.is_any_anchored_end());
    assert!(!result.is_match_empty());
}

