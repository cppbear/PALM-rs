pub fn union_prefixes(&mut self, expr: &Hir) -> bool {
        let mut lits = self.to_empty();
        prefixes(expr, &mut lits);
        !lits.is_empty() && !lits.contains_empty() && self.union(lits)
    }