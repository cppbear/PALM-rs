// Answer 0

#[test]
fn test_exec_at_reverse_with_next_si_max_and_quit_after_match() {
    let mut prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![0; 256],
        only_utf8: false,
        is_bytes: true,
        is_dfa: true,
        is_reverse: true,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 100,
    };
    
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut fsm = Fsm {
        prog: &prog,
        start: 0,
        at: 5,
        quit_after_match: true,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let text = b"hello";
    let mut qcur = SparseSet {
        dense: vec![0; 10],
        sparse: vec![0; 10],
        size: 5,
    };
    
    let mut qnext = SparseSet {
        dense: vec![0; 10],
        sparse: vec![0; 10],
        size: 5,
    };

    let result = fsm.exec_at_reverse(&mut qcur, &mut qnext, text);
    match result {
        Result::Match(_) => assert!(true),
        _ => panic!("Expected a match result with quit after match set."),
    }
}

#[test]
fn test_exec_at_reverse_with_at_reaching_zero() {
    let mut prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![0; 256],
        only_utf8: false,
        is_bytes: true,
        is_dfa: true,
        is_reverse: true,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 100,
    };
    
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![],
        start_states: vec![],
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
        cache: &mut cache,
    };

    let text = b"hello";
    let mut qcur = SparseSet {
        dense: vec![0; 10],
        sparse: vec![0; 10],
        size: 5,
    };

    let mut qnext = SparseSet {
        dense: vec![0; 10],
        sparse: vec![0; 10],
        size: 5,
    };

    let result = fsm.exec_at_reverse(&mut qcur, &mut qnext, text);
    match result {
        Result::NoMatch(_) => assert!(true),
        _ => panic!("Expected no match result with at reaching zero."),
    }
}

#[test]
#[should_panic]
fn test_exec_at_reverse_with_panic_conditions() {
    let mut prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![0; 256],
        only_utf8: false,
        is_bytes: true,
        is_dfa: true,
        is_reverse: true,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 100,
    };

    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut fsm = Fsm {
        prog: &prog,
        start: 0,
        at: 1,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let text = &[1, 2, 3];
    let mut qcur = SparseSet {
        dense: vec![0; 10],
        sparse: vec![0; 10],
        size: 5,
    };

    let mut qnext = SparseSet {
        dense: vec![0; 10],
        sparse: vec![0; 10],
        size: 5,
    };

    fsm.exec_at_reverse(&mut qcur, &mut qnext, text);
}

