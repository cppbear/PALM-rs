// Answer 0

#[test]
fn test_is_match_at_unanchored() {
    let text: &[u8] = &[72, 101, 108, 108, 111]; // "Hello"
    let start: usize = 0;

    let cache = RefCell::new(ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    });

    let captures = Arc::new(HashMap::new());
    let program = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: captures.clone(),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 0,
    };

    let ro = Arc::new(ExecReadOnly {
        res: vec!["Hello".to_string()],
        nfa: program.clone(),
        dfa: program.clone(),
        dfa_reverse: program.clone(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::Literal(MatchLiteralType::Unanchored),
    });

    let exec = ExecNoSync {
        ro: &ro,
        cache: &cache,
    };

    exec.is_match_at(text, start);
}

#[test]
fn test_is_match_at_anchored_start() {
    let text: &[u8] = &[72, 101, 108, 108, 111]; // "Hello"
    let start: usize = 0;

    let cache = RefCell::new(ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    });

    let captures = Arc::new(HashMap::new());
    let program = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: captures.clone(),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: true,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 0,
    };

    let ro = Arc::new(ExecReadOnly {
        res: vec!["Hello".to_string()],
        nfa: program.clone(),
        dfa: program.clone(),
        dfa_reverse: program.clone(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::Literal(MatchLiteralType::AnchoredStart),
    });

    let exec = ExecNoSync {
        ro: &ro,
        cache: &cache,
    };

    exec.is_match_at(text, start);
}

#[test]
fn test_is_match_at_anchored_end() {
    let text: &[u8] = &[72, 101, 108, 108, 111]; // "Hello"
    let start: usize = 5; // End of "Hello"

    let cache = RefCell::new(ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    });

    let captures = Arc::new(HashMap::new());
    let program = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: captures.clone(),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: true,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 0,
    };

    let ro = Arc::new(ExecReadOnly {
        res: vec!["Hello".to_string()],
        nfa: program.clone(),
        dfa: program.clone(),
        dfa_reverse: program.clone(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::Literal(MatchLiteralType::AnchoredEnd),
    });

    let exec = ExecNoSync {
        ro: &ro,
        cache: &cache,
    };

    exec.is_match_at(text, start);
}

