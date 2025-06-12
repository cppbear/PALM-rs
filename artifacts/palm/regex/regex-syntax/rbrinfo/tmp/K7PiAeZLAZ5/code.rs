fn alternate_literals<F: FnMut(&Hir, &mut Literals)>(
    es: &[Hir],
    lits: &mut Literals,
    mut f: F,
) {
    let mut lits2 = lits.to_empty();
    for e in es {
        let mut lits3 = lits.to_empty();
        lits3.set_limit_size(lits.limit_size() / 5);
        f(e, &mut lits3);
        if lits3.is_empty() || !lits2.union(lits3) {
            // If we couldn't find suffixes for *any* of the
            // alternates, then the entire alternation has to be thrown
            // away and any existing members must be frozen. Similarly,
            // if the union couldn't complete, stop and freeze.
            lits.cut();
            return;
        }
    }
    if !lits.cross_product(&lits2) {
        lits.cut();
    }
}