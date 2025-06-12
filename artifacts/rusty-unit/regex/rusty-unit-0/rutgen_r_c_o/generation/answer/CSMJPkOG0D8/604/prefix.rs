// Answer 0

#[test]
fn test_exec_at_case_1() {
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
        prog: &program,
        start: 2,
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
    
    let text = b"test input";
    fsm.exec_at(&mut qcur, &mut qnext, text);
}

#[test]
fn test_exec_at_case_2() {
    let program = Program {
        insts: vec![Inst::new()],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 0,
        byte_classes: vec![0; 257],
        only_utf8: false,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 256,
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
        prog: &program,
        start: 3,
        at: 1,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    };

    let mut qcur = SparseSet {
        dense: vec![5],
        sparse: vec![5],
        size: 1,
    };
    
    let mut qnext = SparseSet {
        dense: vec![6],
        sparse: vec![6],
        size: 1,
    };
    
    let text = b"example text";
    fsm.exec_at(&mut qcur, &mut qnext, text);
}

#[test]
fn test_exec_at_case_3() {
    let program = Program {
        insts: vec![],
        matches: vec![],
        captures: vec![],
        capture_name_idx: Arc::new(HashMap::new()),
        start: 10,
        byte_classes: vec![0; 128],
        only_utf8: true,
        is_bytes: false,
        is_dfa: true,
        is_reverse: false,
        is_anchored_start: true,
        is_anchored_end: false,
        has_unicode_word_boundary: true,
        prefixes: LiteralSearcher::new(),
        dfa_size_limit: 50,
    };
    
    let mut cache = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::new(),
        states: vec![],
        start_states: vec![],
        stack: vec![],
        flush_count: 5,
        size: 0,
    };
    
    let mut fsm = Fsm {
        prog: &program,
        start: 1,
        at: 0,
        quit_after_match: true,
        last_match_si: STATE_DEAD,
        last_cache_flush: 1,
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
    
    let text = b"short input";
    fsm.exec_at(&mut qcur, &mut qnext, text);
}

