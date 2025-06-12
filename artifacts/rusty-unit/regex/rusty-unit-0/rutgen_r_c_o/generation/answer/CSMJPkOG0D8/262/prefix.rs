// Answer 0

#[test]
fn test_exec_at_with_non_empty_text() {
    let program = Program {
        insts: vec![],
        matches: vec![],
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
        states: vec![State { data: vec![0; 256].into_boxed_slice() }],
        start_states: vec![0],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let text: &[u8] = b"test input for DFA execution";
    let mut fsm = Fsm {
        prog: &program,
        start: STATE_MAX,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let mut qcur = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };
    let mut qnext = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };

    let _ = fsm.exec_at(&mut qcur, &mut qnext, text);
}

#[test]
fn test_exec_at_with_max_bound_prev_si_and_next_si() {
    let program = Program {
        insts: vec![],
        matches: vec![],
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
        states: vec![State { data: vec![0; 256].into_boxed_slice() }],
        start_states: vec![0],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let text: &[u8] = b"sample input for testing";
    let mut fsm = Fsm {
        prog: &program,
        start: STATE_MAX,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let mut qcur = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };
    let mut qnext = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };

    let _ = fsm.exec_at(&mut qcur, &mut qnext, text);
}

#[test]
fn test_exec_at_empty_qcur_and_qnext() {
    let program = Program {
        insts: vec![],
        matches: vec![],
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
        states: vec![State { data: vec![0; 256].into_boxed_slice() }],
        start_states: vec![0],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let text: &[u8] = b"non-empty input";
    let mut fsm = Fsm {
        prog: &program,
        start: STATE_MAX,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let mut qcur = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };
    let mut qnext = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };

    let _ = fsm.exec_at(&mut qcur, &mut qnext, text);
}

#[test]
fn test_exec_at_with_state_unknown() {
    let program = Program {
        insts: vec![],
        matches: vec![],
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
        states: vec![State { data: vec![0; 256].into_boxed_slice() }],
        start_states: vec![0],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let text: &[u8] = b"test to trigger STATE_UNKNOWN";
    let mut fsm = Fsm {
        prog: &program,
        start: STATE_MAX,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let mut qcur = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };
    let mut qnext = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };

    let _ = fsm.exec_at(&mut qcur, &mut qnext, text);
}

