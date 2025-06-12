fn remove_complete(&mut self) -> Vec<Literal> {
        let mut base = vec![];
        for lit in mem::replace(&mut self.lits, vec![]) {
            if lit.is_cut() {
                self.lits.push(lit);
            } else {
                base.push(lit);
            }
        }
        base
    }