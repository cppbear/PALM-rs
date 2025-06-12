// Answer 0

#[test]
fn test_exec_at_reverse_case_1() {
    let mut prog = Program {
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
        prog: &mut prog,
        start: STATE_START,
        at: 20,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let mut qcur = SparseSet {
        dense: vec![0; 10],
        sparse: vec![0; 10],
        size: 10,
    };

    let mut qnext = SparseSet {
        dense: vec![0; 10],
        sparse: vec![0; 10],
        size: 10,
    };

    let text = b"abcde";
    let _ = fsm.exec_at_reverse(&mut qcur, &mut qnext, text);
}

#[test]
fn test_exec_at_reverse_case_2() {
    let mut prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![0; 300],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
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
        prog: &mut prog,
        start: STATE_START,
        at: 10,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let mut qcur = SparseSet {
        dense: vec![0; 10],
        sparse: vec![0; 10],
        size: 10,
    };

    let mut qnext = SparseSet {
        dense: vec![0; 10],
        sparse: vec![0; 10],
        size: 10,
    };

    let text = b"xyz";
    let _ = fsm.exec_at_reverse(&mut qcur, &mut qnext, text);
}

#[test]
fn test_exec_at_reverse_case_3() {
    let mut prog = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![0; 300],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
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
        prog: &mut prog,
        start: STATE_START,
        at: 25,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let mut qcur = SparseSet {
        dense: vec![0; 10],
        sparse: vec![0; 10],
        size: 10,
    };

    let mut qnext = SparseSet {
        dense: vec![0; 10],
        sparse: vec![0; 10],
        size: 10,
    };

    let text = b"mnopqr";
    let _ = fsm.exec_at_reverse(&mut qcur, &mut qnext, text);
}

