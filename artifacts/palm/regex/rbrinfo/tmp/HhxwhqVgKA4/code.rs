pub fn leads_to_match(&self, pc: usize) -> bool {
        if self.matches.len() > 1 {
            // If we have a regex set, then we have more than one ending
            // state, so leading to one of those states is generally
            // meaningless.
            return false;
        }
        match self[self.skip(pc)] {
            Inst::Match(_) => true,
            _ => false,
        }
    }