fn cached_state(
        &mut self,
        q: &SparseSet,
        mut state_flags: StateFlags,
        current_state: Option<&mut StatePtr>,
    ) -> Option<StatePtr> {
        // If we couldn't come up with a non-empty key to represent this state,
        // then it is dead and can never lead to a match.
        //
        // Note that inst_flags represent the set of empty width assertions
        // in q. We use this as an optimization in exec_byte to determine when
        // we should follow epsilon transitions at the empty string preceding
        // the current byte.
        let key = match self.cached_state_key(q, &mut state_flags) {
            None => return Some(STATE_DEAD),
            Some(v) => v,
        };
        // In the cache? Cool. Done.
        if let Some(&si) = self.cache.compiled.get(&key) {
            return Some(si);
        }
        // If the cache has gotten too big, wipe it.
        if self.approximate_size() > self.prog.dfa_size_limit
            && !self.clear_cache_and_save(current_state)
            {
                // Ooops. DFA is giving up.
                return None;
            }
        // Allocate room for our state and add it.
        self.add_state(key)
    }