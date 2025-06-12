// Answer 0

#[test]
fn test_exec_nfa_backtrack() {
    let matches = &mut [false; 1000];
    let slots = &mut [Slot::default(); 100]; 
    let text = b"test text for nfa execution";
    let start = 0;

    let exec_instance = ExecNoSync {
        ro: &Arc::new(ExecReadOnly {
            res: vec!["regex1".into()],
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: LiteralSearcher::default(),
            match_type: MatchType::default(),
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };

    exec_instance.exec_nfa(MatchNfaType::Backtrack, matches, slots, false, text, start);
}

#[test]
fn test_exec_nfa_pikevm() {
    let matches = &mut [false; 1000];
    let slots = &mut [Slot::default(); 100]; 
    let text = b"example text for NFA execution";
    let start = 5;

    let exec_instance = ExecNoSync {
        ro: &Arc::new(ExecReadOnly {
            res: vec!["regex2".into()],
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: LiteralSearcher::default(),
            match_type: MatchType::default(),
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };

    exec_instance.exec_nfa(MatchNfaType::PikeVM, matches, slots, true, text, start);
}

#[test]
#[should_panic]
fn test_exec_nfa_auto_unreachable() {
    let matches = &mut [false; 1000];
    let slots = &mut [Slot::default(); 100]; 
    let text = b"should panic when auto execution is forced";
    let start = 0;

    let exec_instance = ExecNoSync {
        ro: &Arc::new(ExecReadOnly {
            res: vec!["regex3".into()],
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: LiteralSearcher::default(),
            match_type: MatchType::default(),
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };

    exec_instance.exec_nfa(MatchNfaType::Auto, matches, slots, false, text, start);
}

#[test]
fn test_exec_nfa_large_inputs() {
    let matches = &mut [false; 1000];
    let slots = &mut [Slot::default(); 100]; 
    let text = b"large input text for nfa with maximum size allowed text input for testing.";
    let start = 0;

    let exec_instance = ExecNoSync {
        ro: &Arc::new(ExecReadOnly {
            res: vec!["regex4".into()],
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: LiteralSearcher::default(),
            match_type: MatchType::default(),
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };

    exec_instance.exec_nfa(MatchNfaType::Backtrack, matches, slots, false, text, start);
}

