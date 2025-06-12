fn add_state(&mut self, state: State) -> Option<StatePtr> {
        // This will fail if the next state pointer exceeds STATE_PTR. In
        // practice, the cache limit will prevent us from ever getting here,
        // but maybe callers will set the cache size to something ridiculous...
        let si = match self.cache.trans.add() {
            None => return None,
            Some(si) => si,
        };
        // If the program has a Unicode word boundary, then set any transitions
        // for non-ASCII bytes to STATE_QUIT. If the DFA stumbles over such a
        // transition, then it will quit and an alternative matching engine
        // will take over.
        if self.prog.has_unicode_word_boundary {
            for b in 128..256 {
                let cls = self.byte_class(Byte::byte(b as u8));
                self.cache.trans.set_next(si, cls, STATE_QUIT);
            }
        }
        // Finally, put our actual state on to our heap of states and index it
        // so we can find it later.
        self.cache.size +=
            self.cache.trans.state_heap_size()
            + (2 * state.data.len())
            + (2 * mem::size_of::<State>())
            + mem::size_of::<StatePtr>();
        self.cache.states.push(state.clone());
        self.cache.compiled.insert(state, si);
        // Transition table and set of states and map should all be in sync.
        debug_assert!(self.cache.states.len()
                      == self.cache.trans.num_states());
        debug_assert!(self.cache.states.len()
                      == self.cache.compiled.len());
        Some(si)
    }