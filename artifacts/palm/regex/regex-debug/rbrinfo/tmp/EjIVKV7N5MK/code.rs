fn literals<F: Fn(&mut Literals, &Hir) -> bool>(
        &self,
        exprs: &[Hir],
        get_literals: F,
    ) -> Literals {
        let mut lits = Some(self.empty_literals());
        for e in exprs {
            lits = lits.and_then(|mut lits| {
                if !get_literals(&mut lits, e) {
                    None
                } else {
                    Some(lits)
                }
            });
        }
        lits.unwrap_or(self.empty_literals())
    }