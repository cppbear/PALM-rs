// Answer 0

#[test]
fn test_suffixes_with_non_empty_group() {
    let mut lits = Literals::empty();
    let mut group_hir = Hir::empty();
    let child_hir = Hir::literal(Literal::Unicode('a'));
    group_hir = Hir::group(group_hir);
    suffixes(&group_hir, &mut lits);
}

#[test]
fn test_suffixes_with_group_of_multiple_literals() {
    let mut lits = Literals::empty();
    let mut group_hir = Hir::empty();
    let child_hir1 = Hir::literal(Literal::Unicode('a'));
    let child_hir2 = Hir::literal(Literal::Byte(1));
    let concat_exprs = vec![child_hir1, child_hir2];
    group_hir = Hir::concat(concat_exprs);
    suffixes(&group_hir, &mut lits);
}

#[test]
fn test_suffixes_with_empty_group() {
    let mut lits = Literals::empty();
    let group_hir = Hir::group(Hir::empty());
    suffixes(&group_hir, &mut lits);
}

#[test]
fn test_suffixes_with_group_and_repetition() {
    let mut lits = Literals::empty();
    let mut group_hir = Hir::empty();
    let child_hir = Hir::repetition(Repetition {
        kind: hir::RepetitionKind::OneOrMore,
        greedy: true,
        hir: Box::new(Hir::literal(Literal::Unicode('b'))),
    });
    group_hir = Hir::group(group_hir);
    suffixes(&child_hir, &mut lits);
}

#[test]
fn test_suffixes_with_group_and_singles() {
    let mut lits = Literals::empty();
    let group_hir = Hir::group(Hir::literal(Literal::Byte(2)));
    suffixes(&group_hir, &mut lits);
}

#[test]
fn test_suffixes_with_group_containing_concat() {
    let mut lits = Literals::empty();
    let child_hir1 = Hir::literal(Literal::Unicode('x'));
    let child_hir2 = Hir::literal(Literal::Unicode('y'));
    let concat_exprs = vec![child_hir1, child_hir2];
    let group_hir = Hir::group(Hir::concat(concat_exprs));
    suffixes(&group_hir, &mut lits);
}

