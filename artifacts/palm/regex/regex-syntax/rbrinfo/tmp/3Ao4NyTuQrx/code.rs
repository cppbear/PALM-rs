pub fn contains_empty(&self) -> bool {
        self.lits.iter().any(|lit| lit.is_empty())
    }