pub fn reverse(
        prog: &'a Program,
        cache: &ProgramCache,
        quit_after_match: bool,
        text: &[u8],
        at: usize,
    ) -> Result<usize> {
        let mut cache = cache.borrow_mut();
        let cache = &mut cache.dfa_reverse;
        let mut dfa = Fsm {
            prog: prog,
            start: 0, // filled in below
            at: at,
            quit_after_match: quit_after_match,
            last_match_si: STATE_UNKNOWN,
            last_cache_flush: at,
            cache: &mut cache.inner,
        };
        let (empty_flags, state_flags) = dfa.start_flags_reverse(text, at);
        dfa.start = match dfa.start_state(
            &mut cache.qcur,
            empty_flags,
            state_flags,
        ) {
            None => return Result::Quit,
            Some(STATE_DEAD) => return Result::NoMatch(at),
            Some(si) => si,
        };
        debug_assert!(dfa.start != STATE_UNKNOWN);
        dfa.exec_at_reverse(&mut cache.qcur, &mut cache.qnext, text)
    }