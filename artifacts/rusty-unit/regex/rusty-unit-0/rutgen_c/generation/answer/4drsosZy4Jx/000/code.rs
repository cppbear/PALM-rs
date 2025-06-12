// Answer 0

#[test]
fn test_concat_empty() {
    let result = Hir::concat(vec![]);
    assert_eq!(result.kind(), &HirKind::Empty);
}

#[test]
fn test_concat_single() {
    let literal = Literal { value: 'a' }; // Assuming Literal struct exists
    let hir = Hir::literal(literal);
    let result = Hir::concat(vec![hir.clone()]);
    assert_eq!(result.kind(), &HirKind::Literal(literal));
}

#[test]
fn test_concat_multiple() {
    let literal_a = Literal { value: 'a' };
    let literal_b = Literal { value: 'b' };
    let hir_a = Hir::literal(literal_a);
    let hir_b = Hir::literal(literal_b);
    let result = Hir::concat(vec![hir_a.clone(), hir_b.clone()]);

    match result.kind() {
        HirKind::Concat(exprs) => {
            assert_eq!(exprs.len(), 2);
            assert_eq!(exprs[0].kind(), &HirKind::Literal(literal_a));
            assert_eq!(exprs[1].kind(), &HirKind::Literal(literal_b));
        },
        _ => panic!("Expected a concatenation"),
    }
}

#[test]
fn test_concat_flattening() {
    let literal_a = Literal { value: 'a' };
    let literal_b = Literal { value: 'b' };
    let hir_a = Hir::literal(literal_a);
    let hir_b = Hir::literal(literal_b);
    
    let concat_hir = Hir::concat(vec![hir_a.clone(), hir_b.clone()]);
    let result = Hir::concat(vec![concat_hir.clone()]);

    match result.kind() {
        HirKind::Concat(exprs) => {
            assert_eq!(exprs.len(), 2);
            assert_eq!(exprs[0].kind(), &HirKind::Literal(literal_a));
            assert_eq!(exprs[1].kind(), &HirKind::Literal(literal_b));
        },
        _ => panic!("Expected a concatenation"),
    }
}

