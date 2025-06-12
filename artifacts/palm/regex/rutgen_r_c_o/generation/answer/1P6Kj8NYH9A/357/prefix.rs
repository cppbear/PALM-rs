// Answer 0

#[test]
fn test_exec_at_reverse_1() {
    struct TestStruct<'a> {
        fsm: Fsm<'a>,
    }

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
        is_reverse: true,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 1024,
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

    let mut sparse_set_current = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };
    
    let mut sparse_set_next = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };

    let text = b"test";
    let at = 1;

    let mut fsm_instance = TestStruct { fsm: Fsm {
        prog: &program,
        start: STATE_START,
        at,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    }};

    fsm_instance.fsm.exec_at_reverse(&mut sparse_set_current, &mut sparse_set_next, text);
}

#[test]
fn test_exec_at_reverse_2() {
    struct TestStruct<'a> {
        fsm: Fsm<'a>,
    }

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
        is_reverse: true,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 1024,
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

    let mut sparse_set_current = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };
    
    let mut sparse_set_next = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };

    let text = b"test";
    let at = 0;

    let mut fsm_instance = TestStruct { fsm: Fsm {
        prog: &program,
        start: STATE_START,
        at,
        quit_after_match: false,
        last_match_si: STATE_UNKNOWN,
        last_cache_flush: 0,
        cache: &mut cache,
    }};

    fsm_instance.fsm.exec_at_reverse(&mut sparse_set_current, &mut sparse_set_next, text);
}

#[test]
fn test_exec_at_reverse_3() {
    struct TestStruct<'a> {
        fsm: Fsm<'a>,
    }

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
        is_reverse: true,
        is_anchored_start: false,
        is_anchored_end: false,
        has_unicode_word_boundary: false,
        prefixes: LiteralSearcher::default(),
        dfa_size_limit: 1024,
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

    let mut sparse_set_current = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };
    
    let mut sparse_set_next = SparseSet {
        dense: vec![],
        sparse: vec![],
        size: 0,
    };

    let text = b"test";
    let at = 1;

    let mut fsm_instance = TestStruct { fsm: Fsm {
        prog: &program,
        start: STATE_START,
        at,
        quit_after_match: false,
        last_match_si: STATE_QUIT,
        last_cache_flush: 0,
        cache: &mut cache,
    }};

    fsm_instance.fsm.exec_at_reverse(&mut sparse_set_current, &mut sparse_set_next, text);
}

