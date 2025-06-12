// Answer 0

#[derive(Debug)]
struct DFA {
    table: Vec<usize>,
    num_byte_classes: usize,
}

impl DFA {
    fn new(table: Vec<usize>, num_byte_classes: usize) -> Self {
        DFA { table, num_byte_classes }
    }

    fn num_states(&self) -> usize {
        self.table.len() / self.num_byte_classes
    }
}

#[test]
fn test_num_states_with_empty_table() {
    let dfa = DFA::new(vec![], 1);
    assert_eq!(dfa.num_states(), 0);
}

#[test]
fn test_num_states_with_single_class() {
    let dfa = DFA::new(vec![0, 1, 2, 3], 1);
    assert_eq!(dfa.num_states(), 4);
}

#[test]
fn test_num_states_with_multiple_classes() {
    let dfa = DFA::new(vec![0, 1, 2, 3, 4, 5], 2);
    assert_eq!(dfa.num_states(), 3);
}

#[test]
fn test_num_states_with_edge_division() {
    let dfa = DFA::new(vec![0, 1, 2, 3, 4], 5);
    assert_eq!(dfa.num_states(), 1);
}

#[test]
fn test_num_states_with_large_table() {
    let dfa = DFA::new((0..100).collect(), 10);
    assert_eq!(dfa.num_states(), 10);
}

