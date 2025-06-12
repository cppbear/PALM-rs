// Answer 0

#[test]
fn test_fmt_empty_dfa() {
    struct TestDFA {
        num_byte_classes: usize,
        num_states_value: usize,
        table: Vec<u8>,
    }

    impl TestDFA {
        fn num_states(&self) -> usize {
            self.num_states_value
        }
    }

    struct TransitionsRow<'a>(&'a [u8]);

    let dfa = TestDFA {
        num_byte_classes: 0,
        num_states_value: 0,
        table: vec![],
    };

    let result = std::format!("{:?}", dfa);
    assert_eq!(result, "{}"); // Expected output for an empty DFA
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_fmt_invalid_dfa_index() {
    struct TestDFA {
        num_byte_classes: usize,
        num_states_value: usize,
        table: Vec<u8>,
    }

    impl TestDFA {
        fn num_states(&self) -> usize {
            self.num_states_value
        }
    }

    struct TransitionsRow<'a>(&'a [u8]);

    let dfa = TestDFA {
        num_byte_classes: 3,
        num_states_value: 1,
        table: vec![0, 1, 2], // Not enough elements for two states
    };

    let _ = std::format!("{:?}", dfa); // This should panic due to index out of bounds on the second iteration
}

