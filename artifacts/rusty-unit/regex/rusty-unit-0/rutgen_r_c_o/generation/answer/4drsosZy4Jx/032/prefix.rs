// Answer 0

#[test]
fn test_concat_empty() {
    let exprs: Vec<Hir> = Vec::new();
    concat(exprs);
}

#[test]
fn test_concat_single_expression() {
    let exprs = vec![Hir::empty()];
    concat(exprs);
}

#[test]
fn test_concat_two_expressions() {
    let exprs = vec![Hir::empty(), Hir::empty()];
    concat(exprs);
}

#[test]
fn test_concat_three_expressions() {
    let exprs = vec![Hir::empty(), Hir::empty(), Hir::empty()];
    concat(exprs);
}

#[test]
fn test_concat_multiple_expressions() {
    let exprs = vec![Hir::empty(), Hir::empty(), Hir::empty(), Hir::empty()];
    concat(exprs);
}

