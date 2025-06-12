pub fn cross_product(&mut self, lits: &Literals) -> bool {
        if lits.is_empty() {
            return true;
        }
        // Check that we make sure we stay in our limits.
        let mut size_after;
        if self.is_empty() || !self.any_complete() {
            size_after = self.num_bytes();
            for lits_lit in lits.literals() {
                size_after += lits_lit.len();
            }
        } else {
            size_after = self.lits.iter().fold(0, |accum, lit| {
                accum + if lit.is_cut() { lit.len() } else { 0 }
            });
            for lits_lit in lits.literals() {
                for self_lit in self.literals() {
                    if !self_lit.is_cut() {
                        size_after += self_lit.len() + lits_lit.len();
                    }
                }
            }
        }
        if size_after > self.limit_size {
            return false;
        }

        let mut base = self.remove_complete();
        if base.is_empty() {
            base = vec![Literal::empty()];
        }
        for lits_lit in lits.literals() {
            for mut self_lit in base.clone() {
                self_lit.extend(&**lits_lit);
                self_lit.cut = lits_lit.cut;
                self.lits.push(self_lit);
            }
        }
        true
    }