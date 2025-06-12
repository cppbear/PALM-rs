// Answer 0

#[test]
fn test_many_matches_at_a_case() {
    let regex_string = "abc";
    let text: &[u8] = b"xyzabcdef";
    let start: usize = 0;
    let mut matches = vec![false; 1];
    
    let prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: true,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: true,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 100,
    };

    let readonly = ExecReadOnly {
        res: vec![regex_string.to_string()],
        nfa: prog.clone(),
        dfa: prog.clone(),
        dfa_reverse: prog.clone(),
        suffixes: LiteralSearcher::new(),
        match_type: MatchType::DfaSuffix,
    };

    let cache = ProgramCache::new();
    
    let exec = ExecNoSync {
        ro: &Arc::new(readonly),
        cache: &cache,
    };

    exec.many_matches_at(&mut matches, text, start);
}

#[test]
fn test_many_matches_at_b_case() {
    let regex_string = "def";
    let text: &[u8] = b"abcdef";
    let start: usize = 1;
    let mut matches = vec![false; 1];

    let prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: true,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: true,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 100,
    };

    let readonly = ExecReadOnly {
        res: vec![regex_string.to_string()],
        nfa: prog.clone(),
        dfa: prog.clone(),
        dfa_reverse: prog.clone(),
        suffixes: LiteralSearcher::new(),
        match_type: MatchType::DfaSuffix,
    };

    let cache = ProgramCache::new();
    
    let exec = ExecNoSync {
        ro: &Arc::new(readonly),
        cache: &cache,
    };

    exec.many_matches_at(&mut matches, text, start);
}

#[test]
fn test_many_matches_at_c_case() {
    let regex_string = "xyz";
    let text: &[u8] = b"xyzabcdefgh";
    let start: usize = 0;
    let mut matches = vec![false; 1];

    let prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: true,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: true,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 100,
    };

    let readonly = ExecReadOnly {
        res: vec![regex_string.to_string()],
        nfa: prog.clone(),
        dfa: prog.clone(),
        dfa_reverse: prog.clone(),
        suffixes: LiteralSearcher::new(),
        match_type: MatchType::Dfa,
    };

    let cache = ProgramCache::new();
    
    let exec = ExecNoSync {
        ro: &Arc::new(readonly),
        cache: &cache,
    };

    exec.many_matches_at(&mut matches, text, start);
}

#[test]
fn test_many_matches_at_edge_case() {
    let regex_string = "abcde";
    let text: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    let start: usize = 0;
    let mut matches = vec![false; 1];

    let prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: true,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: true,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 100,
    };

    let readonly = ExecReadOnly {
        res: vec![regex_string.to_string()],
        nfa: prog.clone(),
        dfa: prog.clone(),
        dfa_reverse: prog.clone(),
        suffixes: LiteralSearcher::new(),
        match_type: MatchType::DfaMany,
    };

    let cache = ProgramCache::new();
    
    let exec = ExecNoSync {
        ro: &Arc::new(readonly),
        cache: &cache,
    };

    exec.many_matches_at(&mut matches, text, start);
}

