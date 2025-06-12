// Answer 0

#[test]
fn test_prefixes_with_group_literal() {
    let lit1 = Literal::new(vec![b'a']);
    let lit2 = Literal::new(vec![b'b']);
    let group_hir = Hir::group(hir::Group { hir: Box::new(Hir::concat(vec![Hir::literal(lit1.clone()), Hir::literal(lit2.clone())])) });
    let mut lits = Literals::empty().set_limit_size(100).set_limit_class(10);
    prefixes(&group_hir, &mut lits);
}

#[test]
fn test_prefixes_with_group_empty() {
    let group_hir = Hir::group(hir::Group { hir: Box::new(Hir::empty()) });
    let mut lits = Literals::empty().set_limit_size(100).set_limit_class(10);
    prefixes(&group_hir, &mut lits);
}

#[test]
fn test_prefixes_with_group_repetition() {
    let lit = Literal::new(vec![b'x']);
    let rep_hir = Hir::repetition(Repetition { kind: hir::RepetitionKind::OneOrMore, greedy: true, hir: Box::new(Hir::literal(lit)) });
    let group_hir = Hir::group(hir::Group { hir: Box::new(rep_hir) });
    let mut lits = Literals::empty().set_limit_size(100).set_limit_class(10);
    prefixes(&group_hir, &mut lits);
}

#[test]
fn test_prefixes_with_group_alternation() {
    let lit1 = Literal::new(vec![b'y']);
    let lit2 = Literal::new(vec![b'z']);
    let alt_hir = Hir::alternation(vec![Hir::literal(lit1.clone()), Hir::literal(lit2.clone())]);
    let group_hir = Hir::group(hir::Group { hir: Box::new(alt_hir) });
    let mut lits = Literals::empty().set_limit_size(100).set_limit_class(10);
    prefixes(&group_hir, &mut lits);
}

#[test]
fn test_prefixes_group_with_cut() {
    let lit = Literal::new(vec![b'c']);
    let mut group_lits = Literals::empty().set_limit_size(50).set_limit_class(5);
    group_lits.add(lit.clone());
    let group_hir = Hir::group(hir::Group { hir: Box::new(Hir::literal(lit)) });
    let mut lits = Literals::empty().set_limit_size(100).set_limit_class(10);
    lits.cut();
    prefixes(&group_hir, &mut lits);
}

