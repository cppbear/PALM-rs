// Answer 0

#[test]
fn test_clear_cache_and_save_empty_cache() {
    struct StatePtr(usize);
    
    struct Cache {
        states: Vec<StatePtr>,
    }

    struct DFA {
        cache: Cache,
    }
    
    impl DFA {
        fn new() -> Self {
            DFA {
                cache: Cache { states: Vec::new() },
            }
        }
        
        fn clear_cache(&mut self) -> bool {
            self.cache.states.clear();
            true
        }
        
        fn state(&self, _si: StatePtr) -> StatePtr {
            StatePtr(0) // Dummy implementation
        }
        
        fn restore_state(&self, _state: StatePtr) -> Option<StatePtr> {
            Some(StatePtr(0)) // Dummy implementation
        }
        
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
                    *si = self.restore_state(cur).unwrap();
                    true
                }
            }
        }
    }

    let mut dfa = DFA::new();
    let mut current_state: Option<&mut StatePtr> = None;

    let result = dfa.clear_cache_and_save(current_state);

    assert_eq!(result, true);
}

