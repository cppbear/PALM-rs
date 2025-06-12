// Answer 0

#[test]
fn test_is_match_at_false_due_to_is_anchor_end_match() {
    let text: Vec<u8> = vec![104, 101, 108, 108, 111]; // "hello"
    let start: usize = 0;

    let program_cache = RefCell::new(ProgramCacheInner {
        pikevm: pikevm::Cache::new(),
        backtrack: backtrack::Cache::new(),
        dfa: dfa::Cache::new(),
        dfa_reverse: dfa::Cache::new(),
    });

    let regex_options = RegexOptions::default();

    let program = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 1024,
    };

    let exec_read_only = Arc::new(ExecReadOnly {
        res: vec!["^hello$".to_string()],
        nfa: program.clone(),
        dfa: program.clone(),
        dfa_reverse: program.clone(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::Dfa,
    });

    let exec = ExecNoSync {
        ro: &exec_read_only,
        cache: &program_cache,
    };

    let result = exec.is_match_at(&text, start);
}

#[test]
fn test_is_match_at_false_due_to_invalid_start() {
    let text: Vec<u8> = vec![108, 111, 114]; // "lor"
    let start: usize = 1;

    let program_cache = RefCell::new(ProgramCacheInner {
        pikevm: pikevm::Cache::new(),
        backtrack: backtrack::Cache::new(),
        dfa: dfa::Cache::new(),
        dfa_reverse: dfa::Cache::new(),
    });

    let regex_options = RegexOptions::default();

    let program = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
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
        dfa_size_limit: 1024,
    };

    let exec_read_only = Arc::new(ExecReadOnly {
        res: vec!["hello".to_string()],
        nfa: program.clone(),
        dfa: program.clone(),
        dfa_reverse: program.clone(),
        suffixes: LiteralSearcher::default(),
        match_type: MatchType::Nfa(MatchNfaType::Auto),
    });

    let exec = ExecNoSync {
        ro: &exec_read_only,
        cache: &program_cache,
    };

    let result = exec.is_match_at(&text, start);
}

