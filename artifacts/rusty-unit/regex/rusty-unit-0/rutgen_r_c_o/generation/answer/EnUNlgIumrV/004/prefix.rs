// Answer 0

#[test]
fn test_find_nfa_no_match_auto() {
    let text: &[u8] = &[0];
    let start: usize = 0;
    let ty = MatchNfaType::Auto;
    let ro = Arc::new(ExecReadOnly {
        res: vec![],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec_no_sync = ExecNoSync { ro: &ro, cache: &cache };
    exec_no_sync.find_nfa(ty, text, start);
}

#[test]
fn test_find_nfa_no_match_backtrack() {
    let text: &[u8] = &[0];
    let start: usize = 0;
    let ty = MatchNfaType::Backtrack;
    let ro = Arc::new(ExecReadOnly {
        res: vec![],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec_no_sync = ExecNoSync { ro: &ro, cache: &cache };
    exec_no_sync.find_nfa(ty, text, start);
}

#[test]
fn test_find_nfa_no_match_pike_vm() {
    let text: &[u8] = &[0];
    let start: usize = 0;
    let ty = MatchNfaType::PikeVM;
    let ro = Arc::new(ExecReadOnly {
        res: vec![],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec_no_sync = ExecNoSync { ro: &ro, cache: &cache };
    exec_no_sync.find_nfa(ty, text, start);
}

#[test]
fn test_find_nfa_empty_text_auto() {
    let text: &[u8] = &[];
    let start: usize = 0;
    let ty = MatchNfaType::Auto;
    let ro = Arc::new(ExecReadOnly {
        res: vec![],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec_no_sync = ExecNoSync { ro: &ro, cache: &cache };
    exec_no_sync.find_nfa(ty, text, start);
}

#[test]
fn test_find_nfa_single_element_text_auto() {
    let text: &[u8] = &[1];
    let start: usize = 0;
    let ty = MatchNfaType::Auto;
    let ro = Arc::new(ExecReadOnly {
        res: vec![],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec_no_sync = ExecNoSync { ro: &ro, cache: &cache };
    exec_no_sync.find_nfa(ty, text, start);
}

