fn captures_nfa_with_match(
        &self,
        slots: &mut [Slot],
        text: &[u8],
        match_start: usize,
        match_end: usize,
    ) -> Option<(usize, usize)> {
        // We can't use match_end directly, because we may need to examine one
        // "character" after the end of a match for lookahead operators. We
        // need to move two characters beyond the end, since some look-around
        // operations may falsely assume a premature end of text otherwise.
        let e = cmp::min(
            next_utf8(text, next_utf8(text, match_end)), text.len());
        self.captures_nfa(slots, &text[..e], match_start)
    }