// Answer 0

fn test_clear_cache_and_save() {
    struct StatePtr;
    struct Cache {
        states: Vec<StatePtr>,
    }

    struct DFA {
        cache: Cache,
    }

    impl DFA {
        fn clear_cache(&mut self) -> bool {
            // Clear the cache and return true for the sake of this test.
            self.cache.states.clear();
            true
        }

        fn state(&self, _si: &StatePtr) -> &StatePtr {
            // Return a reference to a StatePtr instance.
            &self.cache.states[0] // Assume there's at least one state
        }

        fn restore_state(&self, _state: StatePtr) -> Option<StatePtr> {
            // Always return Some for the sake of this test.
            Some(StatePtr)
        }

        fn clear_cache_and_save(
            &mut self,
            current_state: Option<&mut StatePtr>,
        ) -> bool {
            if self.cache.states.is_empty() {
                return true;
            }
            match current_state {
                None => self.clear_cache(),
                Some(si) => {
                    let cur = self.state(*si).clone();
                    if !self.clear_cache() {
                        return false;
                    }
                    *si = self.restore_state(cur).unwrap();
                    true
                }
            }
        }
    }

    let mut state_ptr = StatePtr;
    let mut dfa = DFA {
        cache: Cache {
            states: vec![state_ptr.clone()],
        },
    };

    assert!(dfa.clear_cache_and_save(Some(&mut state_ptr)));
}

fn test_clear_cache_and_save_empty_cache() {
    struct StatePtr;

    struct Cache {
        states: Vec<StatePtr>,
    }

    struct DFA {
        cache: Cache,
    }

    impl DFA {
        fn clear_cache(&mut self) -> bool {
            true
        }

        fn state(&self, _si: &StatePtr) -> &StatePtr {
            &self.cache.states[0]
        }

        fn restore_state(&self, _state: StatePtr) -> Option<StatePtr> {
            Some(StatePtr)
        }

        fn clear_cache_and_save(
            &mut self,
            current_state: Option<&mut StatePtr>,
        ) -> bool {
            if self.cache.states.is_empty() {
                return true;
            }
            match current_state {
                None => self.clear_cache(),
                Some(si) => {
                    let cur = self.state(*si).clone();
                    if !self.clear_cache() {
                        return false;
                    }
                    *si = self.restore_state(cur).unwrap();
                    true
                }
            }
        }
    }

    let mut dfa = DFA {
        cache: Cache {
            states: vec![],
        },
    };

    assert!(dfa.clear_cache_and_save(None));
}

