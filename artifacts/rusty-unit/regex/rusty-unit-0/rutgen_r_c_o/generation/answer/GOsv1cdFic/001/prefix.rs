// Answer 0

#[test]
fn test_forward_many_case_1() {
    let prog = Program {
        insts: vec![Inst::Match(0), Inst::Match(1)],
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
        dfa_size_limit: 512,
    };

    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![],
        start_states: vec![STATE_DEAD; 64], 
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let cache = ProgramCache {
        dfa: cache_inner.clone(),
        backtrack: backtrack::Cache::default(),
        pikevm: pikevm::Cache::default(),
    };

    let mut matches = vec![false; prog.matches.len()];
    let text = b"testtext";

    let result = Fsm::forward_many(&prog, &cache, &mut matches, text, 0);
}

#[test]
fn test_forward_many_case_2() {
    let prog = Program {
        insts: vec![Inst::Match(0), Inst::Match(1)],
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
        dfa_size_limit: 512,
    };

    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![],
        start_states: vec![STATE_DEAD; 64], 
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let cache = ProgramCache {
        dfa: cache_inner.clone(),
        backtrack: backtrack::Cache::default(),
        pikevm: pikevm::Cache::default(),
    };

    let mut matches = vec![false; prog.matches.len()];
    let text = b"another test text";

    let result = Fsm::forward_many(&prog, &cache, &mut matches, text, 0);
}

#[test]
fn test_forward_many_edge_case_1() {
    let prog = Program {
        insts: vec![Inst::Match(0), Inst::Match(1)],
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
        dfa_size_limit: 512,
    };

    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![],
        start_states: vec![
            STATE_DEAD,
            STATE_UNKNOWN,
            STATE_DEAD,
            STATE_UNKNOWN,
            STATE_DEAD,
        ],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let cache = ProgramCache {
        dfa: cache_inner.clone(),
        backtrack: backtrack::Cache::default(),
        pikevm: pikevm::Cache::default(),
    };

    let mut matches = vec![false; prog.matches.len()];
    let text = b"";

    let result = Fsm::forward_many(&prog, &cache, &mut matches, text, 0);
}

#[test]
fn test_forward_many_reverse_case() {
    let prog = Program {
        insts: vec![Inst::Match(0)],
        matches: vec![0],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: true,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 512,
    };

    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![],
        start_states: vec![STATE_DEAD; 64],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let cache = ProgramCache {
        dfa: cache_inner.clone(),
        backtrack: backtrack::Cache::default(),
        pikevm: pikevm::Cache::default(),
    };

    let mut matches = vec![false; prog.matches.len()];
    let text = b"backward test";

    let result = Fsm::forward_many(&prog, &cache, &mut matches, text, 0);
}

