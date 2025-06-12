// Answer 0

#[test]
fn test_forward_valid_match() {
    let prog = Program {
        insts: vec![/* some valid instructions */],
        matches: vec![0],
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
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 100,
    };

    let mut cache_inner = ProgramCacheInner {
        pikevm: Default::default(),
        backtrack: Default::default(),
        dfa: dfa::Cache::new(),
        dfa_reverse: dfa::Cache::new(),
    };
    let cache = ProgramCache::new(&mut cache_inner);

    let text = b"some input text";
    let at = 0;

    forward(&prog, &cache, true, text, at);
}

#[test]
fn test_forward_no_match() {
    let prog = Program {
        insts: vec![/* some valid instructions */],
        matches: vec![0],
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
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 100,
    };

    let mut cache_inner = ProgramCacheInner {
        pikevm: Default::default(),
        backtrack: Default::default(),
        dfa: dfa::Cache::new(),
        dfa_reverse: dfa::Cache::new(),
    };
    let cache = ProgramCache::new(&mut cache_inner);

    let text = b"some other input";
    let at = 0;

    forward(&prog, &cache, false, text, at);
}

#[test]
fn test_forward_no_start_state() {
    let prog = Program {
        insts: vec![/* some valid instructions that don't start */],
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
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 100,
    };

    let mut cache_inner = ProgramCacheInner {
        pikevm: Default::default(),
        backtrack: Default::default(),
        dfa: dfa::Cache::new(),
        dfa_reverse: dfa::Cache::new(),
    };
    let cache = ProgramCache::new(&mut cache_inner);

    let text = b"test string";
    let at = 0;

    forward(&prog, &cache, true, text, at);
}

#[test]
fn test_forward_edge_case_empty_text() {
    let prog = Program {
        insts: vec![/* valid instructions for empty input */],
        matches: vec![0],
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
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 100,
    };

    let mut cache_inner = ProgramCacheInner {
        pikevm: Default::default(),
        backtrack: Default::default(),
        dfa: dfa::Cache::new(),
        dfa_reverse: dfa::Cache::new(),
    };
    let cache = ProgramCache::new(&mut cache_inner);

    let text = b"";
    let at = 0;

    forward(&prog, &cache, false, text, at);
}

