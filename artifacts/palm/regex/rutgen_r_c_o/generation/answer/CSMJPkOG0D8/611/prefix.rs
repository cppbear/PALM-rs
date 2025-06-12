// Answer 0

#[test]
fn test_exec_at_no_match_and_quit() {
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
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 0,
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
        prog: &program,
        start: STATE_UNKNOWN,
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

    let text: &[u8] = &[];
    fsm.exec_at(&mut qcur, &mut qnext, text);
}

#[test]
fn test_exec_at_partial_match_with_no_quit() {
    let program = Program {
        insts: vec![Inst::Match],
        matches: vec![0],
        captures: vec![Some("capture".to_string())],
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
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 0,
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
        prog: &program,
        start: STATE_UNKNOWN,
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

    let text: &[u8] = &[1];
    fsm.exec_at(&mut qcur, &mut qnext, text);
}

