// Answer 0

#[derive(Default)]
struct DFA {
    table: Vec<u8>,
    num_byte_classes: usize,
}

impl DFA {
    fn num_states(&self) -> usize {
        self.table.len() / self.num_byte_classes
    }
}

#[test]
fn test_num_states_with_nonzero_classes() {
    let dfa = DFA {
        table: vec![1, 2, 3, 4, 5, 6], // 6 entries
        num_byte_classes: 2,          // 2 classes
    };
    assert_eq!(dfa.num_states(), 3); // 6 / 2 = 3
}

#[test]
fn test_num_states_with_single_class() {
    let dfa = DFA {
        table: vec![1, 2], // 2 entries
        num_byte_classes: 1, // 1 class
    };
    assert_eq!(dfa.num_states(), 2); // 2 / 1 = 2
}

#[test]
fn test_num_states_with_empty_table() {
    let dfa = DFA {
        table: Vec::new(), // 0 entries
        num_byte_classes: 1, // 1 class
    };
    assert_eq!(dfa.num_states(), 0); // 0 / 1 = 0
}

#[test]
#[should_panic] // This will panic as division by zero isn't handled
fn test_num_states_with_zero_classes() {
    let dfa = DFA {
        table: vec![1, 2, 3], // 3 entries
        num_byte_classes: 0,   // 0 classes
    };
    let _ = dfa.num_states(); // This will cause panic due to division by zero
}

