fn shortest_nfa_type(
        &self,
        ty: MatchNfaType,
        text: &[u8],
        start: usize,
    ) -> Option<usize> {
        let mut slots = [None, None];
        if self.exec_nfa(ty, &mut [false], &mut slots, true, text, start) {
            slots[1]
        } else {
            None
        }
    }