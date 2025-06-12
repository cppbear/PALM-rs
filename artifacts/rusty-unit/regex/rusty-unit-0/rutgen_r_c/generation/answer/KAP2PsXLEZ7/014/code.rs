// Answer 0

#[test]
fn test_cached_state_key_with_save_and_split() {
    let inst_save = Inst::Save(InstSave {});
    let inst_split = Inst::Split(InstSplit {});
    let program = Program {
        insts: vec![inst_save.clone(), inst_split.clone()],
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
        dfa_size_limit: 10,
    };

    let mut state_flags = StateFlags(0);
    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut fsm = Fsm {
        prog: &program,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache_inner,
    };

    let mut sparse_set = SparseSet {
        dense: vec![0],
        sparse: vec![],
        size: 1,
    };

    let result = fsm.cached_state_key(&sparse_set, &mut state_flags);
    assert!(result.is_some());
}

#[test]
fn test_cached_state_key_with_empty_state() {
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
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 10,
    };

    let mut state_flags = StateFlags(0);
    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut fsm = Fsm {
        prog: &program,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache_inner,
    };

    let mut sparse_set = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };

    let result = fsm.cached_state_key(&sparse_set, &mut state_flags);
    assert!(result.is_none());
}

