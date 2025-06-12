fn find_nfa(
        &self,
        ty: MatchNfaType,
        text: &[u8],
        start: usize,
    ) -> Option<(usize, usize)> {
        let mut slots = [None, None];
        if self.exec_nfa(ty, &mut [false], &mut slots, false, text, start) {
            match (slots[0], slots[1]) {
                (Some(s), Some(e)) => Some((s, e)),
                _ => None,
            }
        } else {
            None
        }
    }