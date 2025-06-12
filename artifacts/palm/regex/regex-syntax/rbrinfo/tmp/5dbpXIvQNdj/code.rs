pub fn cross_add(&mut self, bytes: &[u8]) -> bool {
        // N.B. This could be implemented by simply calling cross_product with
        // a literal set containing just `bytes`, but we can be smarter about
        // taking shorter prefixes of `bytes` if they'll fit.
        if bytes.is_empty() {
            return true;
        }
        if self.lits.is_empty() {
            let i = cmp::min(self.limit_size, bytes.len());
            self.lits.push(Literal::new(bytes[..i].to_owned()));
            self.lits[0].cut = i < bytes.len();
            return !self.lits[0].is_cut();
        }
        let size = self.num_bytes();
        if size + self.lits.len() >= self.limit_size {
            return false;
        }
        let mut i = 1;
        while size + (i * self.lits.len()) <= self.limit_size
            && i < bytes.len() {
            i += 1;
        }
        for lit in &mut self.lits {
            if !lit.is_cut() {
                lit.extend(&bytes[..i]);
                if i < bytes.len() {
                    lit.cut();
                }
            }
        }
        true
    }