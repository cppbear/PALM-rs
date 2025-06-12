// Answer 0

#[derive(Default)]
struct Cache {
    states: Vec<String>,
}

struct StatePtr(usize);

struct DFA {
    cache: Cache,
}

impl DFA {
    fn clear_cache(&mut self) -> bool {
        self.cache.states.clear();
        true // Simulating successful clear
    }

    fn state(&self, si: StatePtr) -> &String {
        &self.cache.states[si.0]
    }

    fn restore_state(&self, state: String) -> Option<StatePtr> {
        self.cache.states.push(state);
        Some(StatePtr(self.cache.states.len() - 1)) // Returning position of newly added state
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

#[test]
fn test_clear_cache_and_save_with_some() {
    let mut dfa = DFA {
        cache: Cache {
            states: vec!["state1".to_string(), "state2".to_string()],
        },
    };
    
    let mut current_state = StatePtr(0);
    let result = dfa.clear_cache_and_save(Some(&mut current_state));
    
    assert!(result);
    assert_eq!(current_state.0, 0); // Since it should restore "state1"
}

#[test]
fn test_clear_cache_and_save_with_none() {
    let mut dfa = DFA {
        cache: Cache {
            states: vec!["state1".to_string(), "state2".to_string()],
        },
    };

    let result = dfa.clear_cache_and_save(None);
    
    assert!(result);
    assert!(dfa.cache.states.is_empty()); // Cache should be cleared
}

#[test]
fn test_clear_cache_and_save_with_empty_cache() {
    let mut dfa = DFA {
        cache: Cache {
            states: vec![],
        },
    };

    let result = dfa.clear_cache_and_save(None);
    
    assert!(result); // Should return true as the cache is empty
}

