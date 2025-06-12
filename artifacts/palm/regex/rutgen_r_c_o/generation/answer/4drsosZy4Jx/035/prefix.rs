// Answer 0

#[test]
fn test_concat_empty() {
    let exprs: Vec<Hir> = Vec::new();
    concat(exprs);
}

#[test]
fn test_concat_single() {
    let lit = Literal::new('a');
    let exprs = vec![Hir::literal(lit)];
    concat(exprs);
}

#[test]
fn test_concat_two_literals() {
    let lit1 = Literal::new('a');
    let lit2 = Literal::new('b');
    let exprs = vec![Hir::literal(lit1), Hir::literal(lit2)];
    concat(exprs);
}

#[test]
fn test_concat_three_literals() {
    let lit1 = Literal::new('a');
    let lit2 = Literal::new('b');
    let lit3 = Literal::new('c');
    let exprs = vec![Hir::literal(lit1), Hir::literal(lit2), Hir::literal(lit3)];
    concat(exprs);
}

#[test]
fn test_concat_multiple_expressions() {
    let lit1 = Literal::new('a');
    let lit2 = Literal::new('b');
    let class = Class::new(vec!['c', 'd']);
    let exprs = vec![Hir::literal(lit1), Hir::literal(lit2), Hir::class(class)];
    concat(exprs);
}

#[test]
fn test_concat_empty_expr() {
    let class = Class::new(vec!['e', 'f']);
    let empty_hir = Hir::empty();
    let exprs = vec![empty_hir, Hir::class(class)];
    concat(exprs);
}

#[test]
fn test_concat_edge_case() {
    let lit1 = Literal::new('x');
    let lit2 = Literal::new('y');
    let lit3 = Literal::new('z');
    let exprs = vec![Hir::literal(lit1), Hir::literal(lit2), Hir::literal(lit3)];
    concat(exprs);
}

