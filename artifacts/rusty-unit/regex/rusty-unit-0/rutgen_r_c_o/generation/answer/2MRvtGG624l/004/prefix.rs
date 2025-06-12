// Answer 0

#[test]
fn test_find_at_with_dfa_suffix_quit() {
    let text: &[u8] = &(0..=1048575).collect::<Vec<u8>>();
    let start = 0;

    let ro = Arc::new(ExecReadOnly {
        res: vec![String::from("test_regex")],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::new(),
        match_type: MatchType::DfaSuffix,
    });

    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    exec.find_at(text, start);
}

#[test]
fn test_find_at_with_non_zero_start_dfa_suffix_quit() {
    let text: &[u8] = &(0..=1048575).collect::<Vec<u8>>();
    let start = 512; // a non-zero start

    let ro = Arc::new(ExecReadOnly {
        res: vec![String::from("test_regex")],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::new(),
        match_type: MatchType::DfaSuffix,
    });

    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    exec.find_at(text, start);
}

#[test]
fn test_find_at_with_edge_case_text_length() {
    let text: &[u8] = &(0..=1048575).collect::<Vec<u8>>(); // max length

    let start = 1048575; // at the end of the text

    let ro = Arc::new(ExecReadOnly {
        res: vec![String::from("test_regex")],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::new(),
        match_type: MatchType::DfaSuffix,
    });

    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    exec.find_at(text, start);
}

