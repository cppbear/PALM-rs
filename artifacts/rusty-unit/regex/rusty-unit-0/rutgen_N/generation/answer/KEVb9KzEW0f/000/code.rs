// Answer 0

#[test]
fn test_continue_past_first_match_reverse() {
    struct Program {
        is_reverse: bool,
        matches: Vec<usize>,
    }

    struct DFA {
        prog: Program,
    }

    let dfa = DFA {
        prog: Program {
            is_reverse: true,
            matches: vec![1, 2, 3],
        },
    };

    assert!(dfa.continue_past_first_match());
}

#[test]
fn test_continue_past_first_match_multiple_matches() {
    struct Program {
        is_reverse: bool,
        matches: Vec<usize>,
    }

    struct DFA {
        prog: Program,
    }

    let dfa = DFA {
        prog: Program {
            is_reverse: false,
            matches: vec![1, 2], // More than one match
        },
    };

    assert!(dfa.continue_past_first_match());
}

#[test]
fn test_continue_past_first_match_single_match() {
    struct Program {
        is_reverse: bool,
        matches: Vec<usize>,
    }

    struct DFA {
        prog: Program,
    }

    let dfa = DFA {
        prog: Program {
            is_reverse: false,
            matches: vec![1], // Only one match
        },
    };

    assert!(!dfa.continue_past_first_match());
}

#[test]
fn test_continue_past_first_match_no_matches() {
    struct Program {
        is_reverse: bool,
        matches: Vec<usize>,
    }

    struct DFA {
        prog: Program,
    }

    let dfa = DFA {
        prog: Program {
            is_reverse: false,
            matches: vec![], // No matches
        },
    };

    assert!(!dfa.continue_past_first_match());
}

