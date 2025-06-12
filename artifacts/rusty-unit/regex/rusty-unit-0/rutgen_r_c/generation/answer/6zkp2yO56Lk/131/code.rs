// Answer 0

#[test]
fn test_alternation_empty() {
    let exprs: Vec<Hir> = vec![];
    let result = Hir::alternation(exprs);
    assert_eq!(result.kind(), &HirKind::Empty);
}

#[test]
fn test_alternation_single() {
    let literal_expr = Hir::literal(Literal::new('a'));
    let exprs = vec![literal_expr.clone()];
    let result = Hir::alternation(exprs);
    assert_eq!(result.kind(), &HirKind::Literal(literal_expr.kind().clone()));
}

#[test]
fn test_alternation_multiple() {
    let literal_expr1 = Hir::literal(Literal::new('a'));
    let literal_expr2 = Hir::literal(Literal::new('b'));
    let exprs = vec![literal_expr1.clone(), literal_expr2.clone()];
    let result = Hir::alternation(exprs);
    if let HirKind::Alternation(ref kinds) = result.kind() {
        assert!(kinds.contains(&literal_expr1));
        assert!(kinds.contains(&literal_expr2));
    } else {
        panic!("Expected Alternation kind");
    }
}

#[test]
fn test_alternation_ancestors_info() {
    let literal_expr = Hir::literal(Literal::new('a'));
    let exprs = vec![literal_expr.clone(), Hir::empty()];
    let result = Hir::alternation(exprs);

    assert!(result.is_always_utf8());
    assert!(result.is_all_assertions());
    assert!(result.is_anchored_start());
    assert!(result.is_anchored_end());
    assert!(!result.is_any_anchored_start());
    assert!(!result.is_any_anchored_end());
    assert!(!result.is_match_empty());
}

