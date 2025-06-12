fn start_state(
        &mut self,
        q: &mut SparseSet,
        empty_flags: EmptyFlags,
        state_flags: StateFlags,
    ) -> Option<StatePtr> {
        // Compute an index into our cache of start states based on the set
        // of empty/state flags set at the current position in the input. We
        // don't use every flag since not all flags matter. For example, since
        // matches are delayed by one byte, start states can never be match
        // states.
        let flagi = {
            (((empty_flags.start as u8) << 0) |
             ((empty_flags.end as u8) << 1) |
             ((empty_flags.start_line as u8) << 2) |
             ((empty_flags.end_line as u8) << 3) |
             ((empty_flags.word_boundary as u8) << 4) |
             ((empty_flags.not_word_boundary as u8) << 5) |
             ((state_flags.is_word() as u8) << 6))
            as usize
        };
        match self.cache.start_states[flagi] {
            STATE_UNKNOWN => {}
            STATE_DEAD => return Some(STATE_DEAD),
            si => return Some(si),
        }
        q.clear();
        let start = usize_to_u32(self.prog.start);
        self.follow_epsilons(start, q, empty_flags);
        // Start states can never be match states because we delay every match
        // by one byte. Given an empty string and an empty match, the match
        // won't actually occur until the DFA processes the special EOF
        // sentinel byte.
        let sp = match self.cached_state(q, state_flags, None) {
            None => return None,
            Some(sp) => self.start_ptr(sp),
        };
        self.cache.start_states[flagi] = sp;
        Some(sp)
    }