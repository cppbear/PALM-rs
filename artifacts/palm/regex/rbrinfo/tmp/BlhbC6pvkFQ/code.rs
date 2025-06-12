fn clear_cache(&mut self) -> bool {
        // Bail out of the DFA if we're moving too "slowly."
        // A heuristic from RE2: assume the DFA is too slow if it is processing
        // 10 or fewer bytes per state.
        // Additionally, we permit the cache to be flushed a few times before
        // caling it quits.
        let nstates = self.cache.states.len();
        if self.cache.flush_count >= 3
            && self.at >= self.last_cache_flush
            && (self.at - self.last_cache_flush) <= 10 * nstates {
            return false;
        }
        // Update statistics tracking cache flushes.
        self.last_cache_flush = self.at;
        self.cache.flush_count += 1;

        // OK, actually flush the cache.
        let start = self.state(self.start & !STATE_START).clone();
        let last_match = if self.last_match_si <= STATE_MAX {
            Some(self.state(self.last_match_si).clone())
        } else {
            None
        };
        self.cache.reset_size();
        self.cache.trans.clear();
        self.cache.states.clear();
        self.cache.compiled.clear();
        for s in &mut self.cache.start_states {
            *s = STATE_UNKNOWN;
        }
        // The unwraps are OK because we just cleared the cache and therefore
        // know that the next state pointer won't exceed STATE_MAX.
        let start_ptr = self.restore_state(start).unwrap();
        self.start = self.start_ptr(start_ptr);
        if let Some(last_match) = last_match {
            self.last_match_si = self.restore_state(last_match).unwrap();
        }
        true
    }