pub fn suffixes(expr: &Hir) -> Literals {
        let mut lits = Literals::empty();
        lits.union_suffixes(expr);
        lits
    }