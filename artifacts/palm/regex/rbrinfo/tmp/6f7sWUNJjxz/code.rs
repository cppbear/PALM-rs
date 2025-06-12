fn exec_byte(
        &mut self,
        qcur: &mut SparseSet,
        qnext: &mut SparseSet,
        mut si: StatePtr,
        b: Byte,
    ) -> Option<StatePtr> {
        use prog::Inst::*;

        // Initialize a queue with the current DFA state's NFA states.
        qcur.clear();
        for ip in self.state(si).inst_ptrs() {
            qcur.insert(ip);
        }

        // Before inspecting the current byte, we may need to also inspect
        // whether the position immediately preceding the current byte
        // satisfies the empty assertions found in the current state.
        //
        // We only need to do this step if there are any empty assertions in
        // the current state.
        let is_word_last = self.state(si).flags().is_word();
        let is_word = b.is_ascii_word();
        if self.state(si).flags().has_empty() {
            // Compute the flags immediately preceding the current byte.
            // This means we only care about the "end" or "end line" flags.
            // (The "start" flags are computed immediately proceding the
            // current byte and is handled below.)
            let mut flags = EmptyFlags::default();
            if b.is_eof() {
                flags.end = true;
                flags.end_line = true;
            } else if b.as_byte().map_or(false, |b| b == b'\n') {
                flags.end_line = true;
            }
            if is_word_last == is_word {
                flags.not_word_boundary = true;
            } else {
                flags.word_boundary = true;
            }
            // Now follow epsilon transitions from every NFA state, but make
            // sure we only follow transitions that satisfy our flags.
            qnext.clear();
            for &ip in &*qcur {
                self.follow_epsilons(usize_to_u32(ip), qnext, flags);
            }
            mem::swap(qcur, qnext);
        }

        // Now we set flags for immediately after the current byte. Since start
        // states are processed separately, and are the only states that can
        // have the StartText flag set, we therefore only need to worry about
        // the StartLine flag here.
        //
        // We do also keep track of whether this DFA state contains a NFA state
        // that is a matching state. This is precisely how we delay the DFA
        // matching by one byte in order to process the special EOF sentinel
        // byte. Namely, if this DFA state containing a matching NFA state,
        // then it is the *next* DFA state that is marked as a match.
        let mut empty_flags = EmptyFlags::default();
        let mut state_flags = StateFlags::default();
        empty_flags.start_line = b.as_byte().map_or(false, |b| b == b'\n');
        if b.is_ascii_word() {
            state_flags.set_word();
        }
        // Now follow all epsilon transitions again, but only after consuming
        // the current byte.
        qnext.clear();
        for &ip in &*qcur {
            match self.prog[ip as usize] {
                // These states never happen in a byte-based program.
                Char(_) | Ranges(_) => unreachable!(),
                // These states are handled when following epsilon transitions.
                Save(_) | Split(_) | EmptyLook(_) => {}
                Match(_) => {
                    state_flags.set_match();
                    if !self.continue_past_first_match() {
                        break;
                    } else if self.prog.matches.len() > 1
                            && !qnext.contains(ip as usize) {
                        // If we are continuing on to find other matches,
                        // then keep a record of the match states we've seen.
                        qnext.insert(ip);
                    }
                }
                Bytes(ref inst) => {
                    if b.as_byte().map_or(false, |b| inst.matches(b)) {
                        self.follow_epsilons(
                            inst.goto as InstPtr, qnext, empty_flags);
                    }
                }
            }
        }

        let cache =
            if b.is_eof() && self.prog.matches.len() > 1 {
                // If we're processing the last byte of the input and we're
                // matching a regex set, then make the next state contain the
                // previous states transitions. We do this so that the main
                // matching loop can extract all of the match instructions.
                mem::swap(qcur, qnext);
                // And don't cache this state because it's totally bunk.
                false
            } else {
                true
            };

        // We've now built up the set of NFA states that ought to comprise the
        // next DFA state, so try to find it in the cache, and if it doesn't
        // exist, cache it.
        //
        // N.B. We pass `&mut si` here because the cache may clear itself if
        // it has gotten too full. When that happens, the location of the
        // current state may change.
        let mut next = match self.cached_state(
            qnext,
            state_flags,
            Some(&mut si),
        ) {
            None => return None,
            Some(next) => next,
        };
        if (self.start & !STATE_START) == next {
            // Start states can never be match states since all matches are
            // delayed by one byte.
            debug_assert!(!self.state(next).flags().is_match());
            next = self.start_ptr(next);
        }
        if next <= STATE_MAX && self.state(next).flags().is_match() {
            next |= STATE_MATCH;
        }
        debug_assert!(next != STATE_UNKNOWN);
        // And now store our state in the current state's next list.
        if cache {
            let cls = self.byte_class(b);
            self.cache.trans.set_next(si, cls, next);
        }
        Some(next)
    }