fn repeat_one_or_more_literals<F: FnMut(&Hir, &mut Literals)>(
    e: &Hir,
    lits: &mut Literals,
    mut f: F,
) {
    f(e, lits);
    lits.cut();
}