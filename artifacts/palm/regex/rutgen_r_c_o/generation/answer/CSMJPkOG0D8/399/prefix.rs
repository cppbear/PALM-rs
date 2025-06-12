// Answer 0

#[test]
fn test_exec_at_case_1() {
    let mut program = Program {
        insts: vec![/* mock instructions */],
        matches: vec![0],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![0, 1, 2],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 10,
    };
    
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![State { data: Box::new([0; 10]) }],
        start_states: vec![1],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };
    
    let mut fsm = Fsm {
        prog: &program,
        start: 0,
        at: 0,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let text: &[u8] = &[0, 1, 2];
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

    let result = fsm.exec_at(&mut qcur, &mut qnext, text);
}

#[test]
fn test_exec_at_case_2() {
    let mut program = Program {
        insts: vec![/* mock instructions */],
        matches: vec![0],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![0, 1],
        only_utf8: false,
        is_bytes: false,
        is_dfa: false,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 10,
    };
    
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![State { data: Box::new([0; 10]) }],
        start_states: vec![1],
        stack: vec![],
        flush_count: 0,
        size: 0,
    };
    
    let mut fsm = Fsm {
        prog: &program,
        start: STATE_MAX,
        at: 2,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let text: &[u8] = &[0, 1];
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

    let result = fsm.exec_at(&mut qcur, &mut qnext, text);
}

