pub fn longest_common_suffix(&self) -> &[u8] {
        if self.is_empty() {
            return &[];
        }
        let lit0 = &*self.lits[0];
        let mut len = lit0.len();
        for lit in &self.lits[1..] {
            len = cmp::min(
                len,
                lit.iter()
                   .rev()
                   .zip(lit0.iter().rev())
                   .take_while(|&(a, b)| a == b)
                   .count());
        }
        &self.lits[0][self.lits[0].len() - len..]
    }