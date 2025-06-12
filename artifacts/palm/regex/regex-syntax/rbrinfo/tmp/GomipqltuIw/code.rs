pub fn reverse(&mut self) {
        for lit in &mut self.lits {
            lit.reverse();
        }
    }