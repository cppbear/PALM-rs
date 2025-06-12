fn match_nfa(
        &self,
        text: &[u8],
        start: usize,
    ) -> bool {
        self.match_nfa_type(MatchNfaType::Auto, text, start)
    }