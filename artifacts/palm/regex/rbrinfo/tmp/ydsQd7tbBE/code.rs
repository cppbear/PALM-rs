fn match_nfa_type(
        &self,
        ty: MatchNfaType,
        text: &[u8],
        start: usize,
    ) -> bool {
        self.exec_nfa(ty, &mut [false], &mut [], true, text, start)
    }