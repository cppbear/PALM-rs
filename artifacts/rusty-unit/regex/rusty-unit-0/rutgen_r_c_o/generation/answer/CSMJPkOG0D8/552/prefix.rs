// Answer 0

#[test]
fn test_exec_at_case_1() {
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![State { data: Box::new([0; 1]) }],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let prog = Program {
        insts: vec![],
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

    let mut fsm = Fsm {
        prog: &prog,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let mut qcur = SparseSet {
        dense: vec![0],
        sparse: vec![0],
        size: 1,
    };

    let mut qnext = SparseSet {
        dense: vec![1],
        sparse: vec![1],
        size: 1,
    };

    let text = vec![0u8; 5];

    let _ = fsm.exec_at(&mut qcur, &mut qnext, &text);
}

#[test]
fn test_exec_at_case_2() {
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![State { data: Box::new([0; 1]) }],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let prog = Program {
        insts: vec![],
        matches: vec![0, 1],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![0u8],
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

    let mut fsm = Fsm {
        prog: &prog,
        start: 0,
        at: 3,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let mut qcur = SparseSet {
        dense: vec![1],
        sparse: vec![0],
        size: 1,
    };

    let mut qnext = SparseSet {
        dense: vec![2],
        sparse: vec![1],
        size: 1,
    };

    let text = vec![0u8, 1, 2, 3];

    let _ = fsm.exec_at(&mut qcur, &mut qnext, &text);
}

