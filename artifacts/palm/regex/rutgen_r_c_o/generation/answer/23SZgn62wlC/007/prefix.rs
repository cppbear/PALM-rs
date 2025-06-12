// Answer 0

#[test]
fn test_exec_dfa_reverse_suffix_case1() {
    let text: Vec<u8> = vec![0];
    let original_start: usize = 1;
    let lcs = FreqyPacked::new(vec![0]);
    let suffixes = LiteralSearcher {
        complete: true,
        lcp: FreqyPacked::new(vec![]), 
        lcs,
        matcher: Matcher::Empty,
    };

    let program_cache = ProgramCache::new();
    let program = Program {
        insts: Vec::new(),
        matches: Vec::new(),
        captures: Vec::new(),
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: Vec::new(),
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: true,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: suffixes.clone(),
        dfa_size_limit: 1024,
    };

    let exec_read_only = Arc::new(ExecReadOnly {
        res: vec!["test".to_string()],
        nfa: program.clone(),
        dfa: program.clone(),
        dfa_reverse: program.clone(),
        suffixes,
        match_type: MatchType::Default,
    });

    let exec_no_sync = ExecNoSync {
        ro: &exec_read_only,
        cache: &program_cache,
    };

    exec_no_sync.exec_dfa_reverse_suffix(&text, original_start);
}

#[test]
#[should_panic]
fn test_exec_dfa_reverse_suffix_case2() {
    let text: Vec<u8> = vec![0, 1, 2];
    let original_start: usize = 2;
    let lcs = FreqyPacked::new(vec![0]);
    let suffixes = LiteralSearcher {
        complete: true,
        lcp: FreqyPacked::new(vec![]), 
        lcs,
        matcher: Matcher::Empty,
    };

    let program_cache = ProgramCache::new();
    let program = Program {
        insts: Vec::new(),
        matches: Vec::new(),
        captures: Vec::new(),
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: Vec::new(),
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: true,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: suffixes.clone(),
        dfa_size_limit: 1024,
    };

    let exec_read_only = Arc::new(ExecReadOnly {
        res: vec!["test".to_string()],
        nfa: program.clone(),
        dfa: program.clone(),
        dfa_reverse: program.clone(),
        suffixes,
        match_type: MatchType::Default,
    });

    let exec_no_sync = ExecNoSync {
        ro: &exec_read_only,
        cache: &program_cache,
    };

    exec_no_sync.exec_dfa_reverse_suffix(&text, original_start);
}

