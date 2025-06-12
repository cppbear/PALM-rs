// Answer 0

#[test]
fn test_cached_state_key_empty_look_with_match() {
    let mut state_flags = StateFlags(1);
    let mut sparse_set = SparseSet {
        dense: vec![0], // ip leading to EmptyLook
        sparse: vec![0],
        size: 1,
    };
    
    let program = Program {
        insts: vec![Inst::EmptyLook(InstEmptyLook {}), Inst::Match(0)],
        matches: vec![0],
        captures: vec![None],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![0],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 256,
    };

    let mut fsm = Fsm {
        prog: &program,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner::default(),
    };

    let _ = fsm.cached_state_key(&sparse_set, &mut state_flags);
}

#[test]
fn test_cached_state_key_empty_look_without_match() {
    let mut state_flags = StateFlags(0);
    let mut sparse_set = SparseSet {
        dense: vec![0], // ip leading to EmptyLook
        sparse: vec![0],
        size: 1,
    };

    let program = Program {
        insts: vec![Inst::EmptyLook(InstEmptyLook {})],
        matches: vec![0],
        captures: vec![None],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![0],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 256,
    };

    let mut fsm = Fsm {
        prog: &program,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner::default(),
    };

    let result = fsm.cached_state_key(&sparse_set, &mut state_flags);
    assert!(result.is_none());
}

#[test]
fn test_cached_state_key_with_bytes_instruction() {
    let mut state_flags = StateFlags(0b00000001); // is_match is true
    let mut sparse_set = SparseSet {
        dense: vec![0, 1], // ip leading to Bytes
        sparse: vec![0, 1],
        size: 2,
    };

    let program = Program {
        insts: vec![
            Inst::Bytes(InstBytes {}), // Bytes instruction
            Inst::Match(0)
        ],
        matches: vec![0],
        captures: vec![None],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![0],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 256,
    };

    let mut fsm = Fsm {
        prog: &program,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner::default(),
    };

    let _ = fsm.cached_state_key(&sparse_set, &mut state_flags);
} 

#[test]
fn test_cached_state_key_no_instructions() {
    let mut state_flags = StateFlags(0);
    let sparse_set = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };

    let program = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![0],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
    };

    let mut fsm = Fsm {
        prog: &program,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut CacheInner::default(),
    };

    let result = fsm.cached_state_key(&sparse_set, &mut state_flags);
    assert!(result.is_none());
}

