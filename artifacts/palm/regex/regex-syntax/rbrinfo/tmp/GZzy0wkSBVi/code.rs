pub fn prefixes(expr: &Hir) -> Literals {
        let mut lits = Literals::empty();
        lits.union_prefixes(expr);
        lits
    }