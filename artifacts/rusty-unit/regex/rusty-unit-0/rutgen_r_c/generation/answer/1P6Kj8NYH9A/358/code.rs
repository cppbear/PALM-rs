// Answer 0

#[test]
fn test_exec_at_reverse_empty_input() {
    let prog = Program {
        insts: vec![],
        matches: vec![],
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
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 0,
    };
    
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };
    
    let mut fsm = Fsm {
        prog: &prog,
        start: STATE_START,
        at: 0,
        quit_after_match: false,
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
    
    let result = fsm.exec_at_reverse(&mut qcur, &mut qnext, &[]);
    assert!(matches!(result, Result::NoMatch(0)));
}

#[test]
fn test_exec_at_reverse_with_match() {
    let prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![0],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: true,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1024,
    };

    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut fsm = Fsm {
        prog: &prog,
        start: STATE_START,
        at: 1,
        quit_after_match: true,
        last_match_si: 0,
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

    // Add mock entries in the transition to simulate a valid run
    // Overriding next_state to return a valid state to simulate behavior
    fsm.next_state = |_qcur, _qnext, _si, _b| Some(2);

    let result = fsm.exec_at_reverse(&mut qcur, &mut qnext, &[1]);
    assert!(matches!(result, Result::Match(0)));
}

#[test]
#[should_panic]
fn test_exec_at_reverse_panic_no_match() {
    let prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![0],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: true,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 1024,
    };

    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut fsm = Fsm {
        prog: &prog,
        start: STATE_START,
        at: 0,
        quit_after_match: false,
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

    fsm.exec_at_reverse(&mut qcur, &mut qnext, &[1]); // Expected to panic
}

