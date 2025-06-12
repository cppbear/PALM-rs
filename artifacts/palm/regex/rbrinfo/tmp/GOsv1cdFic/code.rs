pub fn forward_many(
        prog: &'a Program,
        cache: &ProgramCache,
        matches: &mut [bool],
        text: &[u8],
        at: usize,
    ) -> Result<usize> {
        debug_assert!(matches.len() == prog.matches.len());
        let mut cache = cache.borrow_mut();
        let cache = &mut cache.dfa;
        let mut dfa = Fsm {
            prog: prog,
            start: 0, // filled in below
            at: at,
            quit_after_match: false,
            last_match_si: STATE_UNKNOWN,
            last_cache_flush: at,
            cache: &mut cache.inner,
        };
        let (empty_flags, state_flags) = dfa.start_flags(text, at);
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
        let result = dfa.exec_at(&mut cache.qcur, &mut cache.qnext, text);
        if result.is_match() {
            if matches.len() == 1 {
                matches[0] = true;
            } else {
                debug_assert!(dfa.last_match_si != STATE_UNKNOWN);
                debug_assert!(dfa.last_match_si != STATE_DEAD);
                for ip in dfa.state(dfa.last_match_si).inst_ptrs() {
                    if let Inst::Match(slot) = dfa.prog[ip] {
                        matches[slot] = true;
                    }
                }
            }
        }
        result
    }