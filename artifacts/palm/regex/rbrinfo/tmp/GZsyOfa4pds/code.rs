fn clear_cache_and_save(
        &mut self,
        current_state: Option<&mut StatePtr>,
    ) -> bool {
        if self.cache.states.is_empty() {
            // Nothing to clear...
            return true;
        }
        match current_state {
            None => self.clear_cache(),
            Some(si) => {
                let cur = self.state(*si).clone();
                if !self.clear_cache() {
                    return false;
                }
                // The unwrap is OK because we just cleared the cache and
                // therefore know that the next state pointer won't exceed
                // STATE_MAX.
                *si = self.restore_state(cur).unwrap();
                true
            }
        }
    }