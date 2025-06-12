// Answer 0

#[test]
fn test_alternation_empty() {
    let exprs: Vec<Hir> = Vec::new();
    let result = Hir::alternation(exprs);
    assert_eq!(result.kind, HirKind::Empty);
}

#[test]
fn test_alternation_single() {
    let single_expr = Hir::empty();
    let exprs = vec![single_expr];
    let result = Hir::alternation(exprs);
    assert!(matches!(result.kind, HirKind::Empty));
}

#[test]
fn test_alternation_multiple() {
    let expr1 = Hir::empty();
    let expr2 = Hir::empty(); // This would be a very simple Hir representation.
    let exprs = vec![expr1.clone(), expr2.clone()];
    
    let result = Hir::alternation(exprs.clone());
    
    match &result.kind {
        HirKind::Alternation(kind_exprs) => {
            assert_eq!(kind_exprs.len(), 2);
            assert_eq!(kind_exprs[0].kind, expr1.kind);
            assert_eq!(kind_exprs[1].kind, expr2.kind);
        },
        _ => panic!("Expected Alternation kind"),
    }

    assert!(result.is_always_utf8());
    assert!(result.is_all_assertions());
    assert!(result.is_anchored_start());
    assert!(result.is_anchored_end());
    assert!(!result.is_any_anchored_start());
    assert!(!result.is_any_anchored_end());
    assert!(!result.is_match_empty());
}

#[test]
fn test_alternation_with_non_utf8() {
    // Create HIR that doesn't set all the attributes to true.
    let mut info1 = HirInfo::new();
    info1.set_always_utf8(false);
    
    let mut info2 = HirInfo::new();
    info2.set_always_utf8(false);
    
    let expr1 = Hir { kind: HirKind::Empty, info: info1 };
    let expr2 = Hir { kind: HirKind::Empty, info: info2 };
    
    let exprs = vec![expr1.clone(), expr2.clone()];
    
    let result = Hir::alternation(exprs.clone());
    
    match &result.kind {
        HirKind::Alternation(kind_exprs) => {
            assert_eq!(kind_exprs.len(), 2);
            assert_eq!(kind_exprs[0].info.is_always_utf8(), false);
            assert_eq!(kind_exprs[1].info.is_always_utf8(), false);
        },
        _ => panic!("Expected Alternation kind"),
    }
    
    assert!(!result.is_always_utf8());
    assert!(!result.is_all_assertions());
    assert!(!result.is_anchored_start());
    assert!(!result.is_anchored_end());
    assert!(!result.is_any_anchored_start());
    assert!(!result.is_any_anchored_end());
    assert!(!result.is_match_empty());
}

