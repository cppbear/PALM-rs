pub fn is_empty(&self) -> bool {
        self.lits.is_empty() || self.lits.iter().all(|lit| lit.is_empty())
    }