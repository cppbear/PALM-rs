// Answer 0

#[test]
fn test_set_next_valid_transition() {
    struct DFA {
        table: Vec<StatePtr>,
    }
    
    type StatePtr = usize;
    
    let mut dfa = DFA {
        table: vec![0; 10], // Pre-allocate a table with 10 entries
    };
    
    let si: StatePtr = 2;
    let cls: usize = 1;
    let next: StatePtr = 5;
    
    dfa.set_next(si, cls, next);
    
    assert_eq!(dfa.table[si + cls], next);
}

#[test]
#[should_panic]
fn test_set_next_out_of_bounds() {
    struct DFA {
        table: Vec<StatePtr>,
    }
    
    type StatePtr = usize;
    
    let mut dfa = DFA {
        table: vec![0; 10],
    };
    
    let si: StatePtr = 9; // Last valid index
    let cls: usize = 2; // Will cause out of bounds access
    let next: StatePtr = 5;
    
    dfa.set_next(si, cls, next); // This should panic due to out-of-bounds access
}

#[test]
fn test_set_next_edge_case() {
    struct DFA {
        table: Vec<StatePtr>,
    }
    
    type StatePtr = usize;
    
    let mut dfa = DFA {
        table: vec![0; 10],
    };
    
    let si: StatePtr = 0;
    let cls: usize = 0;
    let next: StatePtr = 4;
    
    dfa.set_next(si, cls, next);
    
    assert_eq!(dfa.table[si + cls], next);
}

