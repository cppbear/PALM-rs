// Answer 0

#[test]
fn test_concat_empty() {
    let exprs: Vec<Hir> = vec![];
    let result = Hir::concat(exprs);
    assert_eq!(result.kind, HirKind::Empty);
}

#[test]
fn test_concat_single_expression() {
    let literal = Literal::new('a');
    let single_expr = Hir::literal(literal);
    let exprs = vec![single_expr.clone()];
    let result = Hir::concat(exprs);
    assert_eq!(result.kind, HirKind::Literal(literal));
}

#[test]
fn test_concat_multiple_expressions() {
    let literal_a = Hir::literal(Literal::new('a'));
    let literal_b = Hir::literal(Literal::new('b'));
    let exprs = vec![literal_a.clone(), literal_b.clone()];
    let result = Hir::concat(exprs);
    if let HirKind::Concat(concats) = result.kind {
        assert_eq!(concats.len(), 2);
        assert_eq!(concats[0], literal_a);
        assert_eq!(concats[1], literal_b);
    } else {
        panic!("Expected HirKind::Concat");
    }
}

#[test]
fn test_concat_multiple_expressions_with_info() {
    let mut info = HirInfo::new();
    info.set_always_utf8(true);
    info.set_all_assertions(true);
    info.set_any_anchored_start(true);
    info.set_any_anchored_end(true);
    info.set_match_empty(true);
    
    let literal_a = Hir::literal(Literal::new('a'));
    let literal_b = Hir::literal(Literal::new('b'));
    let exprs = vec![literal_a.clone(), literal_b.clone()];
    
    let result = Hir::concat(exprs);
    
    assert!(result.is_always_utf8());
    assert!(result.is_all_assertions());
    assert!(result.is_any_anchored_start());
    assert!(result.is_any_anchored_end());
    assert!(result.is_match_empty());
}

