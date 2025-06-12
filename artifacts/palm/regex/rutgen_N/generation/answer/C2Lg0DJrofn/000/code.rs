// Answer 0

#[derive(Debug)]
struct Prog {
    is_reverse: bool,
    prefixes: Vec<String>,
    is_anchored_start: bool,
}

struct DFA {
    prog: Prog,
}

impl DFA {
    fn has_prefix(&self) -> bool {
        !self.prog.is_reverse
        && !self.prog.prefixes.is_empty()
        && !self.prog.is_anchored_start
    }
}

#[test]
fn test_has_prefix_true() {
    let dfa = DFA {
        prog: Prog {
            is_reverse: false,
            prefixes: vec!["prefix1".to_string()],
            is_anchored_start: false,
        },
    };
    assert!(dfa.has_prefix());
}

#[test]
fn test_has_prefix_reverse() {
    let dfa = DFA {
        prog: Prog {
            is_reverse: true,
            prefixes: vec!["prefix1".to_string()],
            is_anchored_start: false,
        },
    };
    assert!(!dfa.has_prefix());
}

#[test]
fn test_has_prefix_empty_prefixes() {
    let dfa = DFA {
        prog: Prog {
            is_reverse: false,
            prefixes: vec![],
            is_anchored_start: false,
        },
    };
    assert!(!dfa.has_prefix());
}

#[test]
fn test_has_prefix_anchored_start() {
    let dfa = DFA {
        prog: Prog {
            is_reverse: false,
            prefixes: vec!["prefix1".to_string()],
            is_anchored_start: true,
        },
    };
    assert!(!dfa.has_prefix());
}

