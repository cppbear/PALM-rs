// Answer 0

#[test]
fn test_match_nfa_with_empty_text() {
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
    let text: &[u8] = &[];
    let start: usize = 0;
    exec.match_nfa(text, start);
}

#[test]
fn test_match_nfa_with_single_char_text() {
    let ro = Arc::new(ExecReadOnly {
        res: vec!["a".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    let text: &[u8] = b"a";
    let start: usize = 0;
    exec.match_nfa(text, start);
}

#[test]
fn test_match_nfa_with_multiple_chars_text() {
    let ro = Arc::new(ExecReadOnly {
        res: vec!["abc".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    let text: &[u8] = b"abc";
    let start: usize = 0;
    exec.match_nfa(text, start);
}

#[test]
fn test_match_nfa_with_start_index_out_of_bounds() {
    let ro = Arc::new(ExecReadOnly {
        res: vec!["hello".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    let text: &[u8] = b"hello";
    let start: usize = 256; // Out of bounds
    exec.match_nfa(text, start);
}

#[test]
fn test_match_nfa_with_full_range_byte_values() {
    let ro = Arc::new(ExecReadOnly {
        res: vec!["test".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    let text: &[u8] = &[0u8; 256]; // All byte values
    let start: usize = 0;
    exec.match_nfa(text, start);
}

#[test]
fn test_match_nfa_with_non_ascii_text() {
    let ro = Arc::new(ExecReadOnly {
        res: vec!["こんにちは".to_string()],
        nfa: Program::default(),
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::default(),
    });
    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };
    let text: &[u8] = b"\xE3\x81\x93\xE3\x82\x93\xE3\x81\xA1\xE3\x81\xAF"; // "こんにちは" in UTF-8
    let start: usize = 0;
    exec.match_nfa(text, start);
}

