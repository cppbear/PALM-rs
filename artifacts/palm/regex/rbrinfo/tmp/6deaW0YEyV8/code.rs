fn restore_state(&mut self, state: State) -> Option<StatePtr> {
        // If we've already stored this state, just return a pointer to it.
        // None will be the wiser.
        if let Some(&si) = self.cache.compiled.get(&state) {
            return Some(si);
        }
        self.add_state(state)
    }