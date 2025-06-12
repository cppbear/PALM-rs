// Answer 0

#[test]
fn test_clear_table() {
    struct DFA {
        table: Vec<u32>,
    }

    impl DFA {
        fn new() -> Self {
            DFA { table: Vec::new() }
        }

        fn clear(&mut self) {
            self.table.clear();
        }
    }

    let mut dfa = DFA::new();
    dfa.table.push(1);
    dfa.table.push(2);
    dfa.clear();
    assert!(dfa.table.is_empty());
}

#[test]
fn test_clear_empty_table() {
    struct DFA {
        table: Vec<u32>,
    }

    impl DFA {
        fn new() -> Self {
            DFA { table: Vec::new() }
        }

        fn clear(&mut self) {
            self.table.clear();
        }
    }

    let mut dfa = DFA::new();
    dfa.clear();
    assert!(dfa.table.is_empty());
}

