// Answer 0

#[test]
fn test_continue_past_first_match_multiple_matches() {
    struct Prog {
        is_reverse: bool,
        matches: Vec<String>,
    }

    struct DFA {
        prog: Prog,
    }

    let dfa = DFA {
        prog: Prog {
            is_reverse: false,
            matches: vec![String::from("match1"), String::from("match2")],
        },
    };

    assert!(dfa.continue_past_first_match());
}

#[test]
fn test_continue_past_first_match_no_matches() {
    struct Prog {
        is_reverse: bool,
        matches: Vec<String>,
    }

    struct DFA {
        prog: Prog,
    }

    let dfa = DFA {
        prog: Prog {
            is_reverse: false,
            matches: vec![],
        },
    };

    assert!(!dfa.continue_past_first_match());
}

#[test]
fn test_continue_past_first_match_single_match() {
    struct Prog {
        is_reverse: bool,
        matches: Vec<String>,
    }

    struct DFA {
        prog: Prog,
    }

    let dfa = DFA {
        prog: Prog {
            is_reverse: false,
            matches: vec![String::from("match1")],
        },
    };

    assert!(!dfa.continue_past_first_match());
}

