// Answer 0

#[test]
fn test_empty_alternation() {
    let exprs: Vec<Hir> = Vec::new();
    alternation(exprs);
}

#[test]
fn test_single_literal_alternation() {
    let literal = Hir::literal(Literal::from_char('a'));
    let exprs = vec![literal];
    alternation(exprs);
}

#[test]
fn test_multiple_literals_alternation() {
    let literal_a = Hir::literal(Literal::from_char('a'));
    let literal_b = Hir::literal(Literal::from_char('b'));
    let literal_c = Hir::literal(Literal::from_char('c'));
    let exprs = vec![literal_a, literal_b, literal_c];
    alternation(exprs);
}

#[test]
fn test_alternation_with_empty() {
    let empty_hir = Hir::empty();
    let literal = Hir::literal(Literal::from_char('a'));
    let exprs = vec![empty_hir, literal];
    alternation(exprs);
}

#[test]
fn test_alternation_combining_groups() {
    let group = Hir::group(Group::new(vec![]));
    let exprs = vec![group, group];
    alternation(exprs);
}

#[test]
fn test_alternation_with_classes() {
    let class_a = Hir::class(Class::new(vec!['a', 'b']));
    let class_b = Hir::class(Class::new(vec!['c', 'd']));
    let exprs = vec![class_a, class_b];
    alternation(exprs);
}

#[test]
fn test_alternation_with_various_exprs() {
    let literal = Hir::literal(Literal::from_char('x'));
    let group = Hir::group(Group::new(vec![literal.clone()]));
    let class = Hir::class(Class::new(vec!['y', 'z']));
    let exprs = vec![literal, group, class];
    alternation(exprs);
}

