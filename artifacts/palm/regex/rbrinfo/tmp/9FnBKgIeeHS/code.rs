fn exec_pikevm(
        &self,
        matches: &mut [bool],
        slots: &mut [Slot],
        quit_after_match: bool,
        text: &[u8],
        start: usize,
    ) -> bool {
        if self.ro.nfa.uses_bytes() {
            pikevm::Fsm::exec(
                &self.ro.nfa,
                self.cache,
                matches,
                slots,
                quit_after_match,
                ByteInput::new(text, self.ro.nfa.only_utf8),
                start)
        } else {
            pikevm::Fsm::exec(
                &self.ro.nfa,
                self.cache,
                matches,
                slots,
                quit_after_match,
                CharInput::new(text),
                start)
        }
    }