// Answer 0

#[test]
fn test_prefixes_with_concat_non_empty() {
    let expr = Hir::from_kind(HirKind::Concat(vec![
        Hir::from_kind(HirKind::Literal(hir::Literal::Unicode('a'))),
        Hir::from_kind(HirKind::Literal(hir::Literal::Unicode('b'))),
    ]));
    let mut lits = Literals::new();
    prefixes(&expr, &mut lits);
    // Assert the expected literals are present after processing.
    assert!(lits.contains("ab"));
}

#[test]
fn test_prefixes_with_concat_of_one() {
    let expr = Hir::from_kind(HirKind::Concat(vec![
        Hir::from_kind(HirKind::Literal(hir::Literal::Unicode('a'))),
    ]));
    let mut lits = Literals::new();
    prefixes(&expr, &mut lits);
    // Assert the expected literal is present.
    assert!(lits.contains("a"));
}

#[test]
fn test_prefixes_with_concat_start_anchor() {
    let expr = Hir::from_kind(HirKind::Concat(vec![
        Hir::from_kind(HirKind::Anchor(hir::Anchor::StartText)),
        Hir::from_kind(HirKind::Literal(hir::Literal::Unicode('c'))),
    ]));
    let mut lits = Literals::new();
    prefixes(&expr, &mut lits);
    // Expected that the anchor will not cut the literals.
    assert!(lits.contains("c"));
}

#[test]
fn test_prefixes_with_concat_fail_cross_product() {
    let expr = Hir::from_kind(HirKind::Concat(vec![
        Hir::from_kind(HirKind::Literal(hir::Literal::Unicode('x'))),
    ]));
    let mut lits = Literals::new();
    let mut lits2 = lits.to_empty();
    prefixes(&expr, &mut lits2);
    // Simulating failure to cross product
    assert!(!lits.cross_product(&lits2));
}

#[test]
fn test_prefixes_with_empty_concat() {
    let expr = Hir::from_kind(HirKind::Concat(vec![]));
    let mut lits = Literals::new();
    prefixes(&expr, &mut lits);
    // Expect no changes since the concat is empty.
    assert!(lits.is_empty());
}

