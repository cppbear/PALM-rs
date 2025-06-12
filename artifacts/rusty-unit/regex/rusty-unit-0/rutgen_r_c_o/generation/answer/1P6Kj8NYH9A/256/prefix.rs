// Answer 0

#[test]
fn test_exec_at_reverse_case1() {
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };
    
    let prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: true,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
    };

    let mut fsm = Fsm {
        prog: &prog,
        start: STATE_START,
        at: 2,
        quit_after_match: true,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let qcur = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };
    let qnext = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };
    let text = b"test";

    fsm.exec_at_reverse(&mut qcur, &mut qnext, text);
}

#[test]
fn test_exec_at_reverse_case2() {
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };
    
    let prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: true,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
    };

    let mut fsm = Fsm {
        prog: &prog,
        start: STATE_START,
        at: 3,
        quit_after_match: true,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let qcur = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };
    let qnext = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };
    let text = b"match";

    fsm.exec_at_reverse(&mut qcur, &mut qnext, text);
}

#[test]
fn test_exec_at_reverse_case3() {
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };
    
    let prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: true,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
    };

    let mut fsm = Fsm {
        prog: &prog,
        start: STATE_START,
        at: 1,
        quit_after_match: true,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let qcur = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };
    let qnext = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };
    let text = b"test_case";

    fsm.exec_at_reverse(&mut qcur, &mut qnext, text);
}

