pub fn longest_common_prefix(&self) -> &[u8] {
        if self.is_empty() {
            return &[];
        }
        let lit0 = &*self.lits[0];
        let mut len = lit0.len();
        for lit in &self.lits[1..] {
            len = cmp::min(
                len,
                lit.iter()
                   .zip(lit0)
                   .take_while(|&(a, b)| a == b)
                   .count());
        }
        &self.lits[0][..len]
    }