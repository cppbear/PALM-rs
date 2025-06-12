// Answer 0

#[test]
fn test_alternation_empty() {
    let exprs: Vec<Hir> = vec![];
    let result = Hir::alternation(exprs);
    assert_eq!(result.kind, HirKind::Empty);
}

#[test]
fn test_alternation_single() {
    let literal_hir = Hir::literal(Literal::from('a')); // Assuming Literal has a suitable constructor
    let exprs = vec![literal_hir.clone()];
    let result = Hir::alternation(exprs);
    assert_eq!(result.kind, HirKind::Literal(literal_hir.kind())); // Assuming kind can be retrieved
}

#[test]
fn test_alternation_multiple() {
    let literal_hir_a = Hir::literal(Literal::from('a'));
    let literal_hir_b = Hir::literal(Literal::from('b'));
    let exprs = vec![literal_hir_a.clone(), literal_hir_b.clone()];
    let result = Hir::alternation(exprs);
    
    if let HirKind::Alternation(ref exprs) = result.kind {
        assert_eq!(exprs.len(), 2);
        assert!(exprs.contains(&literal_hir_a));
        assert!(exprs.contains(&literal_hir_b));
    } else {
        panic!("Expected Alternation kind");
    }
}

