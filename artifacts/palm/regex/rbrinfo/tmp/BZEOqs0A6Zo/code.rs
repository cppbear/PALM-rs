fn exec_nfa(
        &self,
        mut ty: MatchNfaType,
        matches: &mut [bool],
        slots: &mut [Slot],
        quit_after_match: bool,
        text: &[u8],
        start: usize,
    ) -> bool {
        use self::MatchNfaType::*;
        if let Auto = ty {
            if backtrack::should_exec(self.ro.nfa.len(), text.len()) {
                ty = Backtrack;
            } else {
                ty = PikeVM;
            }
        }
        match ty {
            Auto => unreachable!(),
            Backtrack => self.exec_backtrack(matches, slots, text, start),
            PikeVM => {
                self.exec_pikevm(
                    matches, slots, quit_after_match, text, start)
            }
        }
    }