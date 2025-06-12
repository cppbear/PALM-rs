// Answer 0

#[test]
fn test_find_at_with_valid_inputs() {
    use std::cell::RefCell;
    use std::sync::Arc;
    use syntax::hir::Literals;
    use prog::Program;

    let cache: ProgramCache = RefCell::new(ProgramCacheInner::default());
    let literals = Literals::new(vec![b'a', b'b', b'c']);
    let program_nfa = Program::default();
    let program_dfa = Program::default();
    let program_dfa_reverse = Program::default();

    let read_only = Arc::new(ExecReadOnly {
        res: vec!["a".to_string()],
        nfa: program_nfa,
        dfa: program_dfa,
        dfa_reverse: program_dfa_reverse,
        suffixes: literals,
        match_type: MatchType::DfaAnchoredReverse,
    });

    let exec = ExecNoSync {
        ro: &read_only,
        cache: &cache,
    };

    let text: &[u8] = &[1, 2, 3]; // Text must be [1-255]
    let start: usize = 0; // start must be [0-2^20-1]

    exec.find_at(text, start);
}

#[test]
fn test_find_at_with_quit_condition() {
    use std::cell::RefCell;
    use std::sync::Arc;
    use syntax::hir::Literals;
    use prog::Program;

    let cache: ProgramCache = RefCell::new(ProgramCacheInner::default());
    let literals = Literals::new(vec![b'd', b'e', b'f']);
    let program_nfa = Program::default();
    let program_dfa = Program::default();
    let program_dfa_reverse = Program::default();

    let read_only = Arc::new(ExecReadOnly {
        res: vec!["d".to_string()],
        nfa: program_nfa,
        dfa: program_dfa,
        dfa_reverse: program_dfa_reverse,
        suffixes: literals,
        match_type: MatchType::DfaAnchoredReverse,
    });

    let exec = ExecNoSync {
        ro: &read_only,
        cache: &cache,
    };

    let text: &[u8] = &[4, 5, 6]; // Text must be [1-255]
    let start: usize = 1; // start must be [0-2^20-1]

    exec.find_at(text, start);
}

#[test]
fn test_find_at_with_edge_case() {
    use std::cell::RefCell;
    use std::sync::Arc;
    use syntax::hir::Literals;
    use prog::Program;

    let cache: ProgramCache = RefCell::new(ProgramCacheInner::default());
    let literals = Literals::new(vec![b'x', b'y', b'z']);
    let program_nfa = Program::default();
    let program_dfa = Program::default();
    let program_dfa_reverse = Program::default();

    let read_only = Arc::new(ExecReadOnly {
        res: vec!["x".to_string()],
        nfa: program_nfa,
        dfa: program_dfa,
        dfa_reverse: program_dfa_reverse,
        suffixes: literals,
        match_type: MatchType::DfaAnchoredReverse,
    });

    let exec = ExecNoSync {
        ro: &read_only,
        cache: &cache,
    };

    let text: &[u8] = &[255, 254, 253]; // Text must be [1-255]
    let start: usize = 1; // start must be [0-2^20-1]

    exec.find_at(text, start);
}

