// Answer 0

#[test]
fn test_repeat_zero_or_one_literals_empty_lits() {
    let expr = Hir { kind: HirKind::Empty, info: HirInfo::default() };
    let mut lits = Literals::empty();
    lits.set_limit_size(50);
    repeat_zero_or_one_literals(&expr, &mut lits, |_, _| {});
}

#[test]
fn test_repeat_zero_or_one_literals_half_limit_size() {
    let expr = Hir { kind: HirKind::Empty, info: HirInfo::default() };
    let mut lits = Literals::empty();
    lits.set_limit_size(100);
    repeat_zero_or_one_literals(&expr, &mut lits, |_, _| {});
}

#[test]
fn test_repeat_zero_or_one_literals_large_limit_size() {
    let expr = Hir { kind: HirKind::Empty, info: HirInfo::default() };
    let mut lits = Literals::empty();
    lits.set_limit_size(100);
    repeat_zero_or_one_literals(&expr, &mut lits, |_, _| {});
}

#[test]
fn test_repeat_zero_or_one_literals_empty_function() {
    let expr = Hir { kind: HirKind::Empty, info: HirInfo::default() };
    let mut lits = Literals::empty();
    lits.set_limit_size(40);
    repeat_zero_or_one_literals(&expr, &mut lits, |_, _| {});
} 

#[test]
fn test_repeat_zero_or_one_literals_cross_product_fail() {
    let expr = Hir { kind: HirKind::Literal, info: HirInfo::default() };
    let mut lits = Literals::empty();
    lits.set_limit_size(30);
    repeat_zero_or_one_literals(&expr, &mut lits, |_, _| {});
} 

#[test]
fn test_repeat_zero_or_one_literals_non_empty() {
    let expr = Hir { kind: HirKind::Literal, info: HirInfo::default() };
    let mut lits = Literals::empty();
    lits.set_limit_size(60);
    let lit = Literal::new(vec![1, 2, 3]);
    lits.add(lit);
    repeat_zero_or_one_literals(&expr, &mut lits, |_, _| {});
} 

#[test]
fn test_repeat_zero_or_one_literals_limit_exceed() {
    let expr = Hir { kind: HirKind::Literal, info: HirInfo::default() };
    let mut lits = Literals::empty();
    lits.set_limit_size(10);
    let lit = Literal::new(vec![1, 2, 3, 4, 5, 6]);
    lits.add(lit);
    repeat_zero_or_one_literals(&expr, &mut lits, |_, _| {});
} 

