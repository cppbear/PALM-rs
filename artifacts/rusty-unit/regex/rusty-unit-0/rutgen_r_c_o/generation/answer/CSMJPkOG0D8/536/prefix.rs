// Answer 0

#[test]
fn test_exec_at_with_next_si_above_max_and_match_found() {
    let program = Program {
        insts: vec![],
        matches: vec![0],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![0; 256],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1024,
    };
    
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![State { data: vec![0u8; 256].into_boxed_slice() }],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut fsm = Fsm {
        prog: &program,
        start: 0,
        at: 0,
        quit_after_match: true,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let mut qcur = SparseSet { dense: vec![], sparse: vec![], size: 0 };
    let mut qnext = SparseSet { dense: vec![], sparse: vec![], size: 0 };
    let text = vec![1];

    let _ = unsafe { fsm.exec_at(&mut qcur, &mut qnext, &text) };
}

#[test]
fn test_exec_at_with_next_si_above_max_and_multiple_matches() {
    let program = Program {
        insts: vec![],
        matches: vec![0],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![0; 256],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1024,
    };
    
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![State { data: vec![0u8; 256].into_boxed_slice() }],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut fsm = Fsm {
        prog: &program,
        start: 0,
        at: 0,
        quit_after_match: true,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let mut qcur = SparseSet { dense: vec![], sparse: vec![], size: 0 };
    let mut qnext = SparseSet { dense: vec![], sparse: vec![], size: 0 };
    let text = vec![1, 2, 3];

    let _ = unsafe { fsm.exec_at(&mut qcur, &mut qnext, &text) };
}

#[test]
fn test_exec_at_with_edge_at_position() {
    let program = Program {
        insts: vec![],
        matches: vec![0],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![0; 256],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1024,
    };
    
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![State { data: vec![0u8; 256].into_boxed_slice() }],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut fsm = Fsm {
        prog: &program,
        start: 0,
        at: 2,
        quit_after_match: true,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let mut qcur = SparseSet { dense: vec![], sparse: vec![], size: 0 };
    let mut qnext = SparseSet { dense: vec![], sparse: vec![], size: 0 };
    let text = vec![1, 2, 3];

    let _ = unsafe { fsm.exec_at(&mut qcur, &mut qnext, &text) };
}

