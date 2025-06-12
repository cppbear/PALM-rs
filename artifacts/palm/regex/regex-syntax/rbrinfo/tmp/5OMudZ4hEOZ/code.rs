pub fn all_complete(&self) -> bool {
        !self.lits.is_empty() && self.lits.iter().all(|l| !l.is_cut())
    }