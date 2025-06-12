pub fn unambiguous_suffixes(&self) -> Literals {
        // This is a touch wasteful...
        let mut lits = self.clone();
        lits.reverse();
        let mut unamb = lits.unambiguous_prefixes();
        unamb.reverse();
        unamb
    }