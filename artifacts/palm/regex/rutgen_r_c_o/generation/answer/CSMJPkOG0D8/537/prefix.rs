// Answer 0

#[test]
fn test_exec_at_with_match_state() {
    let prog = Program {
        insts: vec![/* instructions that lead to STATE_MATCH */],
        matches: vec![1, 2],
        captures: vec![None, Some("group1".to_string())],
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
        prefixes: LiteralSearcher::new(/* initialization */),
        dfa_size_limit: 1024,
    };
    
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(/* initialization */),
        states: vec![State { data: vec![0; 256].into_boxed_slice() }],
        start_states: vec![0],
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

    let text = (0..255).collect::<Vec<u8>>();
    let mut qcur = SparseSet {
        dense: vec![0],
        sparse: vec![0],
        size: 1,
    };
    let mut qnext = SparseSet {
        dense: vec![0],
        sparse: vec![0],
        size: 1,
    };

    let _ = fsm.exec_at(&mut qcur, &mut qnext, &text);
}

#[test]
fn test_exec_at_with_multiple_matches() {
    let prog = Program {
        insts: vec![/* instructions appropriate for multiple matches */],
        matches: vec![1, 2],
        captures: vec![None, Some("group2".to_string())],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![1; 256],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(/* initialization */),
        dfa_size_limit: 2048,
    };

    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(/* initialization */),
        states: vec![State { data: vec![1; 256].into_boxed_slice() }],
        start_states: vec![1],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut fsm = Fsm {
        prog: &prog,
        start: STATE_START,
        at: 1,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let text = (1..255).collect::<Vec<u8>>();
    let mut qcur = SparseSet {
        dense: vec![1],
        sparse: vec![1],
        size: 1,
    };
    let mut qnext = SparseSet {
        dense: vec![1],
        sparse: vec![1],
        size: 1,
    };

    let _ = fsm.exec_at(&mut qcur, &mut qnext, &text);
}

