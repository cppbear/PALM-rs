// Answer 0

#[test]
fn test_repeat_zero_or_one_literals_empty_lits3() {
    let e = Hir { kind: HirKind::Empty, info: HirInfo::default() };
    let mut lits = Literals::empty().set_limit_size(2);
    let mut f = |_: &Hir, _: &mut Literals| {};

    repeat_zero_or_one_literals(&e, &mut lits, f);
}

#[test]
fn test_repeat_zero_or_one_literals_cross_product_false() {
    let e = Hir { kind: HirKind::Empty, info: HirInfo::default() };
    let mut lits = Literals::empty().set_limit_size(5);
    lits.add(Literal::new(vec![1, 2]));
    lits.add(Literal::new(vec![3, 4]));

    let mut f = |_: &Hir, _: &mut Literals| {
        let mut empty_lits = Literals::empty().set_limit_size(2);
        empty_lits.add(Literal::new(vec![5, 6]));
    };

    repeat_zero_or_one_literals(&e, &mut lits, f);
}

