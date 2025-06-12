// Answer 0

#[test]
fn test_concat_empty() {
    let exprs: Vec<Hir> = vec![];
    let result = Hir::concat(exprs);
    assert_eq!(result.kind(), &HirKind::Empty);
}

#[test]
fn test_concat_single_expr() {
    struct Literal; // Placeholder for actual Literal struct
    let single_expr = Hir::literal(Literal);
    let exprs = vec![single_expr.clone()];
    let result = Hir::concat(exprs);
    assert_eq!(result, single_expr);
}

#[test]
fn test_concat_multiple_exprs() {
    struct Literal; // Placeholder for actual Literal struct
    let expr1 = Hir::literal(Literal);
    let expr2 = Hir::literal(Literal);
    let exprs = vec![expr1.clone(), expr2.clone()];
    
    let result = Hir::concat(exprs);
    
    match &result.kind() {
        HirKind::Concat(children) => {
            assert_eq!(children.len(), 2);
            assert_eq!(children[0], expr1);
            assert_eq!(children[1], expr2);
        },
        _ => panic!("Expected a Concatenation"),
    };

    assert!(result.is_always_utf8());
    assert!(result.is_all_assertions());
    assert!(result.is_match_empty());
}

