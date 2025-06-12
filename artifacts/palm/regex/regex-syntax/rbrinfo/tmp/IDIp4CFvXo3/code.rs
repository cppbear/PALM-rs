pub fn any_complete(&self) -> bool {
        self.lits.iter().any(|lit| !lit.is_cut())
    }