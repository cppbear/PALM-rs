// Answer 0

#[test]
fn test_cached_state_key_valid_case() {
    use std::sync::Arc;

    let inst1 = prog::Inst::Bytes(0);
    let inst2 = prog::Inst::EmptyLook(prog::InstEmptyLook {});
    let prog = Program {
        insts: vec![inst1.clone(), inst2.clone(), prog::Inst::Match(0)],
        matches: vec![0],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: true,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: prog::LiteralSearcher {},
        dfa_size_limit: 10,
    };

    let mut state_flags = StateFlags(0b00000001); // sets is_match to true
    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Default::default(),
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut sparse_set = SparseSet {
        dense: vec![0, 1, 2],
        sparse: vec![0, 1, 2],
        size: 3,
    };

    let mut fsm = Fsm {
        prog: &prog,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: 1,
        last_cache_flush: 0,
        cache: &mut cache_inner,
    };

    let result = fsm.cached_state_key(&sparse_set, &mut state_flags);
    assert!(result.is_some());
    if let Some(state) = result {
        assert_eq!(state.data.len(), 3);
    }
}

#[test]
#[should_panic]
fn test_cached_state_key_panic_due_to_invalid_ip() {
    use std::sync::Arc;

    let inst1 = prog::Inst::Bytes(0);
    let prog = Program {
        insts: vec![inst1.clone(), prog::Inst::Match(0)],
        matches: vec![0],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: true,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: prog::LiteralSearcher {},
        dfa_size_limit: 10,
    };

    let mut state_flags = StateFlags(0b00000001); // sets is_match to true
    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Default::default(),
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut sparse_set = SparseSet {
        dense: vec![0, 1],
        sparse: vec![0, 1],
        size: 2,
    };

    let mut fsm = Fsm {
        prog: &prog,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: 1,
        last_cache_flush: 0,
        cache: &mut cache_inner,
    };

    fsm.cached_state_key(&sparse_set, &mut state_flags); // This will panic due to ip invalid access
}

