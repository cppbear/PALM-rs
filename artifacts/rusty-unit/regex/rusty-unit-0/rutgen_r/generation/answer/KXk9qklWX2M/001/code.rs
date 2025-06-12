// Answer 0

#[test]
fn test_state_valid_pointer() {
    struct StateCache {
        states: Vec<u8>,
    }

    struct DFA {
        cache: StateCache,
        num_byte_classes: usize,
    }

    impl DFA {
        fn num_byte_classes(&self) -> usize {
            self.num_byte_classes
        }

        fn state(&self, si: usize) -> &u8 {
            &self.cache.states[si / self.num_byte_classes()]
        }
    }

    let cache = StateCache {
        states: vec![0, 1, 2, 3, 4, 5],
    };
    let dfa = DFA {
        cache,
        num_byte_classes: 1,
    };

    assert_eq!(*dfa.state(0), 0);
    assert_eq!(*dfa.state(1), 1);
    assert_eq!(*dfa.state(2), 2);
}

#[test]
#[should_panic]
fn test_state_out_of_bounds() {
    struct StateCache {
        states: Vec<u8>,
    }

    struct DFA {
        cache: StateCache,
        num_byte_classes: usize,
    }

    impl DFA {
        fn num_byte_classes(&self) -> usize {
            self.num_byte_classes
        }

        fn state(&self, si: usize) -> &u8 {
            &self.cache.states[si / self.num_byte_classes()]
        }
    }

    let cache = StateCache {
        states: vec![0, 1, 2],
    };
    let dfa = DFA {
        cache,
        num_byte_classes: 1,
    };

    // This will panic as index 3 is out of bounds for the states vector
    let _ = dfa.state(3);
}

#[test]
fn test_state_with_multiple_byte_classes() {
    struct StateCache {
        states: Vec<u8>,
    }

    struct DFA {
        cache: StateCache,
        num_byte_classes: usize,
    }

    impl DFA {
        fn num_byte_classes(&self) -> usize {
            self.num_byte_classes
        }

        fn state(&self, si: usize) -> &u8 {
            &self.cache.states[si / self.num_byte_classes()]
        }
    }

    let cache = StateCache {
        states: vec![0, 1, 2, 3, 4, 5, 6],
    };
    let dfa = DFA {
        cache,
        num_byte_classes: 2,
    };

    assert_eq!(*dfa.state(0), 0);
    assert_eq!(*dfa.state(1), 0);
    assert_eq!(*dfa.state(2), 1);
    assert_eq!(*dfa.state(3), 1);
    assert_eq!(*dfa.state(4), 2);
    assert_eq!(*dfa.state(5), 2);
}

