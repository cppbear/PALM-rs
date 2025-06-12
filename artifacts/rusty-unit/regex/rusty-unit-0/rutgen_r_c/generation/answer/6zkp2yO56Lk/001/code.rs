// Answer 0

#[test]
fn test_alternation_empty() {
    let exprs: Vec<Hir> = Vec::new();
    let result = Hir::alternation(exprs);
    assert_eq!(result.kind, HirKind::Empty);
}

#[test]
fn test_alternation_single() {
    let lit = Literal::new('a');
    let single_hir = Hir::literal(lit);
    let exprs = vec![single_hir.clone()];
    let result = Hir::alternation(exprs);
    assert_eq!(result.kind, HirKind::Literal(lit));
}

#[test]
fn test_alternation_multiple() {
    let lit_a = Hir::literal(Literal::new('a'));
    let lit_b = Hir::literal(Literal::new('b'));
    let exprs = vec![lit_a.clone(), lit_b.clone()];
    let result = Hir::alternation(exprs);
    
    match result.kind {
        HirKind::Alternation(ref inner) => {
            assert_eq!(inner.len(), 2);
            assert!(inner.contains(&lit_a));
            assert!(inner.contains(&lit_b));
        },
        _ => panic!("Expected HirKind::Alternation"),
    }
}

#[test]
fn test_alternation_all_conditions() {
    let lit_a = Hir::literal(Literal::new('a'));
    let lit_b = Hir::literal(Literal::new('b'));
    let exprs = vec![lit_a.clone(), lit_b.clone()];
    let result = Hir::alternation(exprs.clone());

    assert!(result.is_always_utf8());
    assert!(result.is_all_assertions());
    assert!(result.is_anchored_start());
    assert!(result.is_anchored_end());
    assert!(!result.is_any_anchored_start());
    assert!(!result.is_any_anchored_end());
    assert!(!result.is_match_empty());
    
    match result.kind {
        HirKind::Alternation(ref inner) => {
            assert_eq!(inner, &exprs);
        },
        _ => panic!("Expected HirKind::Alternation"),
    }
}

