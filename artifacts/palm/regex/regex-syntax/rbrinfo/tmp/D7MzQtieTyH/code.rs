pub fn union_suffixes(&mut self, expr: &Hir) -> bool {
        let mut lits = self.to_empty();
        suffixes(expr, &mut lits);
        lits.reverse();
        !lits.is_empty() && !lits.contains_empty() && self.union(lits)
    }