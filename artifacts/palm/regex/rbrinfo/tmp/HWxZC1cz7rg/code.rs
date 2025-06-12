fn slots_len(&self) -> usize {
        self.ro.nfa.captures.len() * 2
    }