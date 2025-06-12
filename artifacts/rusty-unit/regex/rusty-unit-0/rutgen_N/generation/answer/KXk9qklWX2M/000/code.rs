// Answer 0

#[test]
fn test_state_valid_pointer() {
    struct MockDFA {
        cache: Cache,
    }

    struct Cache {
        states: Vec<State>,
    }

    struct State;

    impl MockDFA {
        fn num_byte_classes(&self) -> usize {
            256 // example number of byte classes
        }

        fn state(&self, si: StatePtr) -> &State {
            &self.cache.states[si as usize / self.num_byte_classes()]
        }
    }

    type StatePtr = usize;

    let states = vec![State, State]; // Create a few states for testing
    let cache = Cache { states };
    let dfa = MockDFA { cache };

    let si = 0; // valid pointer
    let state_ref = dfa.state(si);
    assert!(state_ref.is_some());
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_state_invalid_pointer() {
    struct MockDFA {
        cache: Cache,
    }

    struct Cache {
        states: Vec<State>,
    }

    struct State;

    impl MockDFA {
        fn num_byte_classes(&self) -> usize {
            256 // example number of byte classes
        }

        fn state(&self, si: StatePtr) -> &State {
            &self.cache.states[si as usize / self.num_byte_classes()]
        }
    }

    type StatePtr = usize;

    let states = vec![State]; // Create a few states for testing
    let cache = Cache { states };
    let dfa = MockDFA { cache };

    let si = 512; // invalid pointer
    dfa.state(si);
}

