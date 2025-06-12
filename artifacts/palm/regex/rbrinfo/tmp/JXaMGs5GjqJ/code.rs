fn shortest_nfa(&self, text: &[u8], start: usize) -> Option<usize> {
        self.shortest_nfa_type(MatchNfaType::Auto, text, start)
    }