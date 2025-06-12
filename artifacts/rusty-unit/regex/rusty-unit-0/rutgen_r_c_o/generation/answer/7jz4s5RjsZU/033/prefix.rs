// Answer 0

#[test]
fn test_read_captures_at_empty_text() {
    let slots = Locations(vec![None, None]);
    let text: &[u8] = b"";
    let start = 0;
    let exec_no_sync = ExecNoSync {
        ro: &Arc::new(ExecReadOnly {
            res: vec![],
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: LiteralSearcher::default(),
            match_type: MatchType::Nothing,
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };
    exec_no_sync.read_captures_at(&mut slots, text, start);
}

#[test]
fn test_read_captures_at_single_character_text() {
    let slots = Locations(vec![None, None]);
    let text: &[u8] = b"a";
    let start = 0;
    let exec_no_sync = ExecNoSync {
        ro: &Arc::new(ExecReadOnly {
            res: vec![],
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: LiteralSearcher::default(),
            match_type: MatchType::Literal(MatchLiteralType::Unanchored),
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };
    exec_no_sync.read_captures_at(&mut slots, text, start);
}

#[test]
fn test_read_captures_at_two_character_text() {
    let slots = Locations(vec![None, None]);
    let text: &[u8] = b"ab";
    let start = 0;
    let exec_no_sync = ExecNoSync {
        ro: &Arc::new(ExecReadOnly {
            res: vec![],
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: LiteralSearcher::default(),
            match_type: MatchType::Literal(MatchLiteralType::AnchoredStart),
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };
    exec_no_sync.read_captures_at(&mut slots, text, start);
}

#[test]
fn test_read_captures_at_long_text() {
    let slots = Locations(vec![None, None]);
    let text: &[u8] = b"0123456789abcde";
    let start = 0;
    let exec_no_sync = ExecNoSync {
        ro: &Arc::new(ExecReadOnly {
            res: vec![],
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: LiteralSearcher::default(),
            match_type: MatchType::Dfa,
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };
    exec_no_sync.read_captures_at(&mut slots, text, start);
}

#[test]
fn test_read_captures_at_repeats() {
    let slots = Locations(vec![None, None]);
    let text: &[u8] = b"abababababababab";
    let start = 0;
    let exec_no_sync = ExecNoSync {
        ro: &Arc::new(ExecReadOnly {
            res: vec![],
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: LiteralSearcher::default(),
            match_type: MatchType::Literal(MatchLiteralType::AnchoredStart),
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };
    exec_no_sync.read_captures_at(&mut slots, text, start);
}

#[test]
fn test_read_captures_at_large_text() {
    let slots = Locations(vec![None, None]);
    let text: Vec<u8> = vec![b'a'; 1 << 20]; // text length = 1 << 20
    let start = 0;
    let exec_no_sync = ExecNoSync {
        ro: &Arc::new(ExecReadOnly {
            res: vec![],
            nfa: Program::default(),
            dfa: Program::default(),
            dfa_reverse: Program::default(),
            suffixes: LiteralSearcher::default(),
            match_type: MatchType::Dfa,
        }),
        cache: &RefCell::new(ProgramCacheInner::default()),
    };
    exec_no_sync.read_captures_at(&mut slots, &text, start);
}

