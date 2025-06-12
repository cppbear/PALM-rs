// Answer 0

#[test]
fn test_cached_state_key_single_match() {
    let insts = vec![Inst::Match(0)];
    let matches = vec![0];
    let captures: Vec<Option<String>> = vec![None];
    let capture_name_idx = Arc::new(HashMap::new());
    
    let prog = Program {
        insts,
        matches,
        captures,
        capture_name_idx,
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
        dfa_size_limit: 0,
    };

    let mut state_flags = StateFlags(0);
    let sparse_set = SparseSet {
        dense: vec![0],
        sparse: vec![0],
        size: 1,
    };

    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: vec![vec![0; 256]; 1],
        states: vec![],
        start_states: vec![0],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };
    
    let mut fsm = Fsm {
        prog: &prog,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache_inner,
    };

    let result = fsm.cached_state_key(&sparse_set, &mut state_flags);
    assert!(result.is_some());
}

#[test]
fn test_cached_state_key_no_match() {
    let insts = vec![Inst::Bytes(0)];
    let matches = vec![0];
    let captures: Vec<Option<String>> = vec![None];
    let capture_name_idx = Arc::new(HashMap::new());
    
    let prog = Program {
        insts,
        matches,
        captures,
        capture_name_idx,
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
        dfa_size_limit: 0,
    };

    let mut state_flags = StateFlags(0);
    let sparse_set = SparseSet {
        dense: vec![0],
        sparse: vec![0],
        size: 1,
    };

    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: vec![vec![0; 256]; 1],
        states: vec![],
        start_states: vec![0],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };
    
    let mut fsm = Fsm {
        prog: &prog,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache_inner,
    };

    let result = fsm.cached_state_key(&sparse_set, &mut state_flags);
    assert!(result.is_none());
}

#[test]
fn test_cached_state_key_empty_transition() {
    let insts = vec![Inst::EmptyLook(0)];
    let matches = vec![0];
    let captures: Vec<Option<String>> = vec![None];
    let capture_name_idx = Arc::new(HashMap::new());
    
    let prog = Program {
        insts,
        matches,
        captures,
        capture_name_idx,
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
        dfa_size_limit: 0,
    };

    let mut state_flags = StateFlags(0);
    let sparse_set = SparseSet {
        dense: vec![0],
        sparse: vec![0],
        size: 1,
    };

    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: vec![vec![0; 256]; 1],
        states: vec![],
        start_states: vec![0],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };
    
    let mut fsm = Fsm {
        prog: &prog,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache_inner,
    };

    let result = fsm.cached_state_key(&sparse_set, &mut state_flags);
    assert!(result.is_none());
}

