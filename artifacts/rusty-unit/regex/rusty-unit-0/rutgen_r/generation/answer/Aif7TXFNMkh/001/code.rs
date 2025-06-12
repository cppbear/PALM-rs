// Answer 0

#[test]
fn test_clear_empty_table() {
    struct DFA {
        table: Vec<u8>,
    }

    let mut dfa = DFA { table: Vec::new() };
    dfa.clear();
    assert!(dfa.table.is_empty());
}

#[test]
fn test_clear_non_empty_table() {
    struct DFA {
        table: Vec<u8>,
    }

    let mut dfa = DFA { table: vec![1, 2, 3] };
    dfa.clear();
    assert!(dfa.table.is_empty());
}

#[test]
fn test_clear_large_table() {
    struct DFA {
        table: Vec<u8>,
    }

    let mut dfa = DFA { table: (0..1_000_000).collect() };
    dfa.clear();
    assert!(dfa.table.is_empty());
}

