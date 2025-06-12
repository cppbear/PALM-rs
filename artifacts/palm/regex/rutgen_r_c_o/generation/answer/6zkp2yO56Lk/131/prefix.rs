// Answer 0

#[test]
fn test_alternation_empty() {
    let exprs: Vec<Hir> = Vec::new();
    alternation(exprs);
}

#[test]
fn test_alternation_single() {
    let literal = Hir::literal(Literal::from('\0'));
    let exprs = vec![literal];
    alternation(exprs);
}

#[test]
fn test_alternation_multiple() {
    let literal1 = Hir::literal(Literal::from('a'));
    let literal2 = Hir::literal(Literal::from('b'));
    let exprs = vec![literal1, literal2];
    alternation(exprs);
}

#[test]
fn test_alternation_multiple_empty() {
    let empty_hir = Hir::empty();
    let exprs = vec![empty_hir.clone(), empty_hir.clone()];
    alternation(exprs);
}

#[test]
fn test_alternation_alternation() {
    let literal1 = Hir::literal(Literal::from('x'));
    let literal2 = Hir::literal(Literal::from('y'));
    let alternation_expr = alternation(vec![literal1.clone(), literal2.clone()]);
    let exprs = vec![alternation_expr];
    alternation(exprs);
}

