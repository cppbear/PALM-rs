fn exec_backtrack(
        &self,
        matches: &mut [bool],
        slots: &mut [Slot],
        text: &[u8],
        start: usize,
    ) -> bool {
        if self.ro.nfa.uses_bytes() {
            backtrack::Bounded::exec(
                &self.ro.nfa,
                self.cache,
                matches,
                slots,
                ByteInput::new(text, self.ro.nfa.only_utf8),
                start)
        } else {
            backtrack::Bounded::exec(
                &self.ro.nfa,
                self.cache,
                matches,
                slots,
                CharInput::new(text),
                start)
        }
    }