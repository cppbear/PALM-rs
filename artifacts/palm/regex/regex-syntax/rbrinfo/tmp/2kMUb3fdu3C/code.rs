pub fn add(&mut self, lit: Literal) -> bool {
        if self.num_bytes() + lit.len() > self.limit_size {
            return false;
        }
        self.lits.push(lit);
        true
    }