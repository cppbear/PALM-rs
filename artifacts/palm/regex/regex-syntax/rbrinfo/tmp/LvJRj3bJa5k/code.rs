pub fn trim_suffix(&self, num_bytes: usize) -> Option<Literals> {
        if self.min_len().map(|len| len <= num_bytes).unwrap_or(true) {
            return None;
        }
        let mut new = self.to_empty();
        for mut lit in self.lits.iter().cloned() {
            let new_len = lit.len() - num_bytes;
            lit.truncate(new_len);
            lit.cut();
            new.lits.push(lit);
        }
        new.lits.sort();
        new.lits.dedup();
        Some(new)
    }