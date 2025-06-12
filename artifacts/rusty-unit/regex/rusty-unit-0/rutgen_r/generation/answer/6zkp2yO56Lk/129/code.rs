// Answer 0

#[test]
fn test_alternation_empty() {
    let exprs: Vec<Hir> = vec![];
    let result = alternation(exprs);
    assert_eq!(result, Hir::empty());
}

#[test]
fn test_alternation_single() {
    let expr = Hir { kind: HirKind::Literal("a".to_string()), info: HirInfo::new() };
    let exprs = vec![expr];
    let result = alternation(exprs);
    assert_eq!(result.kind, HirKind::Literal("a".to_string()));
}

#[test]
fn test_alternation_multiple() {
    let expr1 = Hir { kind: HirKind::Literal("a".to_string()), info: HirInfo::new() };
    let expr2 = Hir { kind: HirKind::Literal("b".to_string()), info: HirInfo::new() };
    let exprs = vec![expr1, expr2];
    let result = alternation(exprs.clone());
    match result.kind {
        HirKind::Alternation(ref alt_exprs) => {
            assert_eq!(alt_exprs.len(), 2);
            assert_eq!(alt_exprs[0].kind, HirKind::Literal("a".to_string()));
            assert_eq!(alt_exprs[1].kind, HirKind::Literal("b".to_string()));
        },
        _ => panic!("Expected HirKind::Alternation"),
    }
}

