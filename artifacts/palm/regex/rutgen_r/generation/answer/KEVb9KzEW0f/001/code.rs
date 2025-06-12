// Answer 0

#[test]
fn test_continue_past_first_match_reverse() {
    struct Prog {
        is_reverse: bool,
        matches: Vec<String>,
    }

    struct DFA {
        prog: Prog,
    }

    let dfa_reverse_single_match = DFA {
        prog: Prog {
            is_reverse: true,
            matches: vec!["match1".to_string()],
        },
    };

    let dfa_reverse_multiple_matches = DFA {
        prog: Prog {
            is_reverse: true,
            matches: vec!["match1".to_string(), "match2".to_string()],
        },
    };

    assert_eq!(dfa_reverse_single_match.continue_past_first_match(), true);
    assert_eq!(dfa_reverse_multiple_matches.continue_past_first_match(), true);
}

#[test]
fn test_continue_past_first_match_non_reverse() {
    struct Prog {
        is_reverse: bool,
        matches: Vec<String>,
    }

    struct DFA {
        prog: Prog,
    }

    let dfa_non_reverse_single_match = DFA {
        prog: Prog {
            is_reverse: false,
            matches: vec!["match1".to_string()],
        },
    };

    let dfa_non_reverse_multiple_matches = DFA {
        prog: Prog {
            is_reverse: false,
            matches: vec!["match1".to_string(), "match2".to_string()],
        },
    };

    assert_eq!(dfa_non_reverse_single_match.continue_past_first_match(), false);
    assert_eq!(dfa_non_reverse_multiple_matches.continue_past_first_match(), true);
}

#[test]
fn test_continue_past_first_match_empty_matches() {
    struct Prog {
        is_reverse: bool,
        matches: Vec<String>,
    }

    struct DFA {
        prog: Prog,
    }

    let dfa_reverse_no_matches = DFA {
        prog: Prog {
            is_reverse: true,
            matches: vec![],
        },
    };

    assert_eq!(dfa_reverse_no_matches.continue_past_first_match(), true);
}

