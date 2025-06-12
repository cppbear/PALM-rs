// Answer 0

#[test]
fn test_forward_many_single_match() {
    let inst = Inst::Match(0);
    let prog = Program {
        insts: vec![inst],
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
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 10,
    };

    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![State { data: vec![b'a', b'b', b'c'].into_boxed_slice() }],
        start_states: vec![2, STATE_UNKNOWN, STATE_DEAD],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let cache = ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    };

    let mut matches = vec![false];
    let text = &[b'a', b'b', b'c'];
    let at = 0;

    forward_many(&prog, &cache, &mut matches, text, at);
}

#[test]
fn test_forward_many_multiple_matches() {
    let inst1 = Inst::Match(0);
    let inst2 = Inst::Match(1);
    let prog = Program {
        insts: vec![inst1, inst2],
        matches: vec![0, 1],
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
        dfa_size_limit: 10,
    };

    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![State { data: vec![b'a', b'b', b'c'].into_boxed_slice() }],
        start_states: vec![2, STATE_UNKNOWN, STATE_DEAD],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let cache = ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    };

    let mut matches = vec![false, false];
    let text = &[b'a', b'b', b'c'];
    let at = 0;

    forward_many(&prog, &cache, &mut matches, text, at);
}

#[test]
fn test_forward_many_no_match() {
    let inst = Inst::Match(0);
    let prog = Program {
        insts: vec![inst],
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
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 10,
    };

    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![State { data: vec![b'x', b'y', b'z'].into_boxed_slice() }],
        start_states: vec![2, STATE_UNKNOWN, STATE_DEAD],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let cache = ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    };

    let mut matches = vec![false];
    let text = &[b'x', b'y', b'z'];
    let at = 0;

    forward_many(&prog, &cache, &mut matches, text, at);
}

#[test]
fn test_forward_many_empty_text() {
    let inst = Inst::Match(0);
    let prog = Program {
        insts: vec![inst],
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
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 10,
    };

    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![State { data: vec![] }],
        start_states: vec![2, STATE_UNKNOWN, STATE_DEAD],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let cache = ProgramCacheInner {
        pikevm: pikevm::Cache::default(),
        backtrack: backtrack::Cache::default(),
        dfa: dfa::Cache::default(),
        dfa_reverse: dfa::Cache::default(),
    };

    let mut matches = vec![false];
    let text: &[u8] = &[];
    let at = 0;

    forward_many(&prog, &cache, &mut matches, text, at);
}

