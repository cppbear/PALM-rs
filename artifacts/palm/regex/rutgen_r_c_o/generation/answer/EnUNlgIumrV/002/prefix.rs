// Answer 0

#[test]
fn test_find_nfa_empty_text() {
    let ro = Arc::new(ExecReadOnly {
        res: vec![],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    let text: &[u8] = b"";
    let start = 0;

    let result = exec.find_nfa(MatchNfaType::Auto, text, start);
}

#[test]
fn test_find_nfa_single_character_text() {
    let ro = Arc::new(ExecReadOnly {
        res: vec![],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    let text: &[u8] = b"a";
    let start = 0;

    let result = exec.find_nfa(MatchNfaType::Auto, text, start);
}

#[test]
fn test_find_nfa_multiple_characters_text() {
    let ro = Arc::new(ExecReadOnly {
        res: vec![],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    let text: &[u8] = b"abcde";
    let start = 0;

    let result = exec.find_nfa(MatchNfaType::Auto, text, start);
}

#[test]
fn test_find_nfa_start_in_middle_text() {
    let ro = Arc::new(ExecReadOnly {
        res: vec![],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    let text: &[u8] = b"abcdef";
    let start = 2;

    let result = exec.find_nfa(MatchNfaType::Auto, text, start);
}

#[test]
fn test_find_nfa_full_length_text() {
    let ro = Arc::new(ExecReadOnly {
        res: vec![],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    let text: &[u8] = b"abcdefghij"; // 10 bytes
    let start = 0;

    let result = exec.find_nfa(MatchNfaType::Auto, text, start);
}

