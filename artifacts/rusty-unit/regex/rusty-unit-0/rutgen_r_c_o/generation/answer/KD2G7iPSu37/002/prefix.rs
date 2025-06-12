// Answer 0

#[test]
fn test_is_match_at_with_nfa_auto() {
    let match_type = MatchType::Nfa(MatchNfaType::Auto);
    let text: &[u8] = b"test string for regex";
    let start = 0;
    
    let ro = Arc::new(ExecReadOnly {
        res: vec![String::from("test")],
        nfa: Program {
            insts: vec![],
            matches: vec![],
            captures: vec![],
            capture_name_idx: Arc::new(HashMap::new()),
            start: 0,
            byte_classes: vec![],
            only_utf8: true,
            is_bytes: false,
            is_dfa: false,
            is_reverse: false,
            is_anchored_start: false,
            is_anchored_end: true,
            has_unicode_word_boundary: false,
            prefixes: LiteralSearcher::default(),
            dfa_size_limit: 0,
        },
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type,
    });

    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    exec.is_match_at(text, start);
}

#[test]
fn test_is_match_at_with_nfa_backtrack() {
    let match_type = MatchType::Nfa(MatchNfaType::Backtrack);
    let text: &[u8] = b"another test string";
    let start = 0;

    let ro = Arc::new(ExecReadOnly {
        res: vec![String::from("another")],
        nfa: Program {
            insts: vec![],
            matches: vec![],
            captures: vec![],
            capture_name_idx: Arc::new(HashMap::new()),
            start: 0,
            byte_classes: vec![],
            only_utf8: true,
            is_bytes: false,
            is_dfa: false,
            is_reverse: false,
            is_anchored_start: false,
            is_anchored_end: true,
            has_unicode_word_boundary: false,
            prefixes: LiteralSearcher::default(),
            dfa_size_limit: 0,
        },
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type,
    });

    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    exec.is_match_at(text, start);
}

#[test]
fn test_is_match_at_with_nfa_pikevm() {
    let match_type = MatchType::Nfa(MatchNfaType::PikeVM);
    let text: &[u8] = b"pike vm regex test";
    let start = 0;

    let ro = Arc::new(ExecReadOnly {
        res: vec![String::from("pike")],
        nfa: Program {
            insts: vec![],
            matches: vec![],
            captures: vec![],
            capture_name_idx: Arc::new(HashMap::new()),
            start: 0,
            byte_classes: vec![],
            only_utf8: true,
            is_bytes: false,
            is_dfa: false,
            is_reverse: false,
            is_anchored_start: false,
            is_anchored_end: true,
            has_unicode_word_boundary: false,
            prefixes: LiteralSearcher::default(),
            dfa_size_limit: 0,
        },
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type,
    });

    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    exec.is_match_at(text, start);
}

#[test]
fn test_is_match_at_with_long_text() {
    let match_type = MatchType::Nfa(MatchNfaType::Auto);
    let text: &[u8] = b"A".repeat(1048576).as_slice(); // generate long text
    let start = 0;

    let ro = Arc::new(ExecReadOnly {
        res: vec![String::from("A")],
        nfa: Program {
            insts: vec![],
            matches: vec![],
            captures: vec![],
            capture_name_idx: Arc::new(HashMap::new()),
            start: 0,
            byte_classes: vec![],
            only_utf8: true,
            is_bytes: false,
            is_dfa: false,
            is_reverse: false,
            is_anchored_start: false,
            is_anchored_end: true,
            has_unicode_word_boundary: false,
            prefixes: LiteralSearcher::default(),
            dfa_size_limit: 0,
        },
        dfa: Program::default(),
        dfa_reverse: Program::default(),
        suffixes: LiteralSearcher::default(),
        match_type,
    });

    let cache = RefCell::new(ProgramCacheInner::default());
    let exec = ExecNoSync { ro: &ro, cache: &cache };

    exec.is_match_at(text, start);
}

