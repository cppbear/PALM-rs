// Answer 0

#[test]
fn test_forward_many_single_match() {
    let insts = vec![Inst::Match(0)];
    let matches = vec![0];
    let program = Program {
        insts,
        matches,
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

    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![],
        start_states: vec![STATE_UNKNOWN; 1 << 6], // Enough flags for tests
        stack: vec![],
        flush_count: 0,
        size: 0,
    };
    
    let mut cache = Cache {
        inner: cache_inner,
        qcur: SparseSet::default(),
        qnext: SparseSet::default(),
    };

    let text = b"test";

    let result = Fsm::forward_many(
        &program,
        &mut cache,
        &mut vec![false],
        text,
        0,
    );

    assert!(result.is_match());
}

#[test]
fn test_forward_many_no_match() {
    let insts = vec![Inst::Match(1)];
    let matches = vec![0, 1];
    let program = Program {
        insts,
        matches,
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

    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![],
        start_states: vec![STATE_UNKNOWN; 1 << 6], // Enough flags for tests
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut cache = Cache {
        inner: cache_inner,
        qcur: SparseSet::default(),
        qnext: SparseSet::default(),
    };

    let text = b"no match here";

    let result = Fsm::forward_many(
        &program,
        &mut cache,
        &mut vec![false, false],
        text,
        0,
    );

    assert!(!result.is_match());
}

#[test]
fn test_forward_many_edge_case_empty_text() {
    let insts = vec![Inst::Match(0)];
    let matches = vec![0];
    let program = Program {
        insts,
        matches,
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

    let mut cache_inner = CacheInner {
        compiled: HashMap::new(),
        trans: Transitions::default(),
        states: vec![],
        start_states: vec![STATE_UNKNOWN; 1 << 6], // Enough flags for tests
        stack: vec![],
        flush_count: 0,
        size: 0,
    };

    let mut cache = Cache {
        inner: cache_inner,
        qcur: SparseSet::default(),
        qnext: SparseSet::default(),
    };

    let text = b"";

    let result = Fsm::forward_many(
        &program,
        &mut cache,
        &mut vec![false],
        text,
        0,
    );

    assert!(!result.is_match());
}

